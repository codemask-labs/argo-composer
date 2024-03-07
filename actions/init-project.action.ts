import { isNotNil } from 'ramda'
import { checkbox, input } from '@inquirer/prompts'
import { isRootDirectoryEmpty, override, writeYamlFile } from '../utils'
import { AddonResource, Application, Kustomization } from '../types'
import { CERT_MANAGER_ADDON_RESOURCE, IMAGE_UPDATER_ADDON_RESOURCE, INGRESS_NGINX_ADDON_RESOURCE, REFLECTOR_ADDON_RESOURCE } from '../addons'

const addAddonApplication = async (rootDirectory: string, addonsProjectName: string, resource: AddonResource<Application>) => {
    const { name: applicationName, resource: applicationResource } = resource
    const application = override(applicationResource, {
        spec: {
            project: addonsProjectName
        }
    })

    await writeYamlFile(`${rootDirectory}/projects/${addonsProjectName}/apps/${applicationName}/application.yaml`, application)
    await writeYamlFile(`${rootDirectory}/projects/${addonsProjectName}/apps/${applicationName}/kustomization.yaml`, {
        resources: ['./application.yaml']
    })

    return {
        path: `./${applicationName}`,
        sourceRepoUrl: application.spec.source.repoURL
    }
}

const addAdditionalApps = async (rootDirectory: string, repoURL: string) => {
    const additionalAppChoices = await checkbox({
        message: 'Do you want to install any additional components?',
        choices: [
            { name: 'ingress-nginx', value: INGRESS_NGINX_ADDON_RESOURCE },
            { name: 'cert-manager', value: CERT_MANAGER_ADDON_RESOURCE },
            { name: 'reflector', value: REFLECTOR_ADDON_RESOURCE },
            { name: 'image-updater', value: IMAGE_UPDATER_ADDON_RESOURCE }
        ]
    })

    if (!additionalAppChoices.length) {
        return null
    }

    const addonsProjectName = await input({ message: 'What project name would you like to use for addons?', default: 'default' })
    const addonPaths = await Promise.all(
        additionalAppChoices.map(resource => addAddonApplication(rootDirectory, addonsProjectName, resource))
    )

    await writeYamlFile(`${rootDirectory}/projects/${addonsProjectName}/apps/kustomization.yaml`, {
        resources: addonPaths.map(({ path }) => path)
    })

    if (addonsProjectName !== 'default') {
        await writeYamlFile(`${rootDirectory}/projects/${addonsProjectName}/project.yaml`, {
            apiVersion: 'argoproj.io/v1alpha1',
            kind: 'AppProject',
            metadata: {
                name: addonsProjectName,
                namespace: 'argocd',
                finalizers: ['resources-finalizer.argocd.argoproj.io']
            },
            spec: {
                description: '<default project description>',
                sourceRepos: [
                    repoURL,
                    ...addonPaths.map(({ sourceRepoUrl }) => sourceRepoUrl)
                ],
                destinations: [
                    {
                        namespace: '*',
                        server: '*'
                    }
                ],
                clusterResourceWhitelist: [
                    {
                        group: '*',
                        kind: '*'
                    }
                ],
                namespaceResourceWhitelist: [
                    {
                        group: '*',
                        kind: '*'
                    }
                ]
            }
        })
    }

    await writeYamlFile(`${rootDirectory}/projects/${addonsProjectName}/kustomization.yaml`, {
        resources: ['./project.yaml','./apps']
    })

    return {
        path: `./${addonsProjectName}`,
        addedInDefaultProject: addonsProjectName === 'default'
    }
}

export const initProjectAction = async () => {
    const rootDirectory = await input({ message: 'What root directory would you like to use for the project?', default: '.' })

    if (!(await isRootDirectoryEmpty(rootDirectory))) {
        throw new Error(`Directory '${rootDirectory}' is not empty`)
    }

    const repoURL = await input({ message: 'What is the base URL of GitHub repository?' })
    const environments = await input({
        message: 'What will be the environment inside your cluster? Provide separated by `,`',
        default: 'dev,prod'
    }).then(environments =>
        environments
            .toLowerCase()
            .split(',')
            .map(environment => environment.trim())
    )

    const addons = await addAdditionalApps(rootDirectory, repoURL)
    const addonsAddedInDefaultProject = addons?.addedInDefaultProject
    const kustomizationResource: Kustomization = {
        resources: ['./apps', './project.yaml']
    }

    await writeYamlFile(`${rootDirectory}/root-app.yaml`, {
        apiVersion: 'argoproj.io/v1alpha1',
        kind: 'Application',
        metadata: {
            name: 'root-app',
            namespace: 'argocd',
            finalizers: ['resources-finalizer.argocd.argoproj.io']
        },
        spec: {
            // https://argo-cd.readthedocs.io/en/stable/user-guide/projects/#the-default-project
            project: 'default',
            revisionHistoryLimit: 0,
            source: {
                path: 'projects',
                repoURL,
                targetRevision: 'main'
            },
            destination: {
                server: 'https://kubernetes.default.svc',
                namespace: 'default'
            },
            syncPolicy: {
                automated: {
                    prune: true,
                    selfHeal: true,
                    retry: {
                        limit: 3,
                        backoff: {
                            duration: '5s',
                            factor: 2,
                            maxDuration: '3m'
                        }
                    }
                },
                syncOptions: [
                    'ApplyOutOfSyncOnly=true',
                    'PruneLast=true'
                ]
            }
        }
    })

    await writeYamlFile(`${rootDirectory}/argo-composer.config.yaml`, {
        repoURL,
        environments
    })

    await writeYamlFile(`${rootDirectory}/projects/default/project.yaml`, {
        apiVersion: 'argoproj.io/v1alpha1',
        kind: 'AppProject',
        metadata: {
            name: 'default',
            namespace: 'argocd'
        },
        spec: {
            sourceRepos: ['*'],
            destinations: [
                {
                    namespace: '*',
                    server: '*'
                }
            ],
            clusterResourceWhitelist: [
                {
                    group: '*',
                    kind: '*'
                }
            ]
        }
    })

    if (!addonsAddedInDefaultProject) {
        await writeYamlFile(`${rootDirectory}/projects/default/kustomization.yaml`, kustomizationResource)
        await writeYamlFile(`${rootDirectory}/projects/default/apps/kustomization.yaml`, { resources: [] })
    }

    await writeYamlFile(`${rootDirectory}/projects/kustomization.yaml`, {
        resources: !addonsAddedInDefaultProject ? ['./default', addons?.path].filter(isNotNil) : [addons.path]
    })
}
