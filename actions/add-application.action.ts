import { join } from 'node:path'
import { ensureDir, readdir } from 'fs-extra'
import { input, select, confirm } from '@inquirer/prompts'
import { getProjectConfig, readYamlFile, writeYamlFile } from '../utils'
import { getApplication, getConfigMap, getDeployment, getHorizontalPodAutoscaler, getIngress, getKustomization, getService } from '../resources'
import { Application, DeepPartial, Kustomization } from '../types'
import { isNotNil, mergeDeepRight } from 'ramda'

export const addApplicationAction = async () => {
    const { mainProjectName, environments, mainRepositoryUrl } = getProjectConfig()

    const projectsDir = join(process.cwd(), 'projects')
    const projectKustomization = await readYamlFile<Kustomization>(`projects/kustomization.yaml`).then(kustomization => ({
        ...kustomization,
        resources: kustomization.resources || []
    }))

    const projectNames = await readdir(projectsDir)
    const appName = await input({
        message: 'What name would you like to use for the app?'
    })

    const projectName = await select({
        message: 'Please select project where you want add your app',
        choices: projectNames.map(projectName => ({
            name: projectName,
            value: projectName
        }))
    })

    const appPort = await input({
        message: 'Provide port of the app'
    })

    const imageURL = await input({
        message: 'Image URL',
        default: 'your-registry.com/your-app'
    })

    const useOverlays = await confirm({
        message: 'Use overlays (multiple environments)?',
        default: true
    })

    const useImageAutoUpdater = await confirm({
        message: 'Use image auto-updater?',
        default: true
    })

    const useHorizontalPodAutoscaler = await confirm({
        message: 'Use Horizontal Pod Autoscaler (HPA)?',
        default: true
    })

    const imageAutoUpdaterAnnotations: Record<string, string> = {
        'argocd-image-updater.argoproj.io/image-list': `i=${imageURL}`,
        'argocd-image-updater.argoproj.io/write-back-method': 'git',
        'argocd-image-updater.argoproj.io/git-branch': 'main',
        'argocd-image-updater.argoproj.io/write-back-target': 'kustomization: .',
        'argocd-image-updater.argoproj.io/i.update-strategy': 'latest',
        'argocd-image-updater.argoproj.io/i.allow-tags': 'regexp:main-*',
        'argocd-image-updater.argoproj.io/i.force-update': 'true'
    }

    const applicationResource = getApplication({
        metadata: {
            name: appName,
            namespace: appName,
            annotations: useImageAutoUpdater ? imageAutoUpdaterAnnotations : {}
        },
        spec: {
            source: {
                repoURL: mainRepositoryUrl,
                path: `projects/${projectName}/apps/${appName}/resources`
            }
        }
    })

    const kustomizationResource = getKustomization({
        resources: [
            'application.yaml'
        ]
    })

    const configMapResource = getConfigMap({})

    const deploymentResource = getDeployment({})

    const ingressResource = getIngress({})

    const serviceResource = getService({})

    const applicationRoot = `projects/${projectName}/apps/${appName}`
    const baseApplicationDir = useOverlays
        ? `${applicationRoot}/base`
        : `${applicationRoot}/resource`

    const applicationKustomization = [
        'configmap.yaml',
        'deployment.yaml',
        'ingress.yaml',
        'service.yaml',
        useHorizontalPodAutoscaler ? 'hpa.yaml' : undefined,
        'kustomization.yaml'
    ]

    if (useHorizontalPodAutoscaler) {
        const hpaResource = getHorizontalPodAutoscaler()

        await writeYamlFile(`${baseApplicationDir}/hpa.yaml`, hpaResource)
    }

    await writeYamlFile(`${baseApplicationDir}/configmap.yaml`, configMapResource)
    await writeYamlFile(`${baseApplicationDir}/deployment.yaml`, deploymentResource)
    await writeYamlFile(`${baseApplicationDir}/ingress.yaml`, ingressResource)
    await writeYamlFile(`${baseApplicationDir}/service.yaml`, serviceResource)
    await writeYamlFile(`${baseApplicationDir}/kustomization.yaml`, {
        resources: applicationKustomization.filter(isNotNil)
    })

    if (useOverlays) {
        const overlayPatches = environments.map(async environment => {
            const root = `${applicationRoot}/overlays/${environment}`
            const application: Application = {
                ...applicationResource,
                metadata: {
                    name: ``,
                    namespace: ``
                }
            }

            return {
                root,
                application
            }
        })

        await Promise.all(overlayPatches)
    }

    await writeYamlFile(
        `projects/${projectName}/apps/${appName}/application.yaml`,
        !useOverlays ? applicationResource : environments.map((environment): Application => {
            const overlay: DeepPartial<Application> = {
                metadata: {
                    namespace: `${projectName}-${appName}-${environment}`
                },
                spec: {
                    source: {
                        path: `projects/${projectName}/apps/${appName}/overlays/${environment}`
                    }
                }
            }

            return mergeDeepRight(applicationResource, overlay as Application)
        })
    )
    await writeYamlFile(`projects/${projectName}/apps/${appName}/kustomization.yaml`, kustomizationResource)
    await writeYamlFile(`projects/${projectName}/apps/kustomization.yaml`, {
        resources: [...projectKustomization.resources, `${appName}`]
    })
}
