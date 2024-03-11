import { isNotNil } from 'ramda'
import { checkbox, input } from '@inquirer/prompts'
import { isRootDirectoryEmpty, override, writeYamlFile } from '../utils'
import { AddonResource, Application, Kustomization } from '../types'
import { CERT_MANAGER_ADDON_RESOURCE, IMAGE_UPDATER_ADDON_RESOURCE, INGRESS_NGINX_ADDON_RESOURCE, REFLECTOR_ADDON_RESOURCE } from '../addons'
import { createAppProject, createApplication } from '../resources/utils'

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
    const addedInDefaultProject = addonsProjectName === 'default'
    const addonApps = await Promise.all(additionalAppChoices.map(resource => addAddonApplication(rootDirectory, addonsProjectName, resource)))

    const appProjectResource = createAppProject({
        name: addonsProjectName,
        sourceRepos: [repoURL, ...addonApps.map(({ sourceRepoUrl }) => sourceRepoUrl)]
    })

    const appsKustomizationResource: Kustomization = {
        resources: addonApps.map(({ path }) => path)
    }

    const appProjectKustomizationResource: Kustomization = {
        resources: ['./project.yaml', './apps']
    }

    await writeYamlFile(`${rootDirectory}/projects/${addonsProjectName}/apps/kustomization.yaml`, appsKustomizationResource)
    await writeYamlFile(`${rootDirectory}/projects/${addonsProjectName}/kustomization.yaml`, appProjectKustomizationResource)
    await writeYamlFile(`${rootDirectory}/projects/${addonsProjectName}/project.yaml`, appProjectResource)

    return {
        path: `./${addonsProjectName}`,
        addedInDefaultProject
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

    const config = {
        repoURL,
        environments
    }

    const addons = await addAdditionalApps(rootDirectory, repoURL)
    const addonsAddedInDefaultProject = addons?.addedInDefaultProject

    const rootAppResource = createApplication({
        name: 'root-app',
        namespace: 'default',
        repoURL
    })

    if (!addonsAddedInDefaultProject) {
        const defaultAppProjectResource = createAppProject({
            name: 'default',
            sourceRepos: [repoURL]
        })

        const kustomizationResource: Kustomization = {
            resources: ['./apps', './project.yaml']
        }

        await writeYamlFile(`${rootDirectory}/projects/default/kustomization.yaml`, kustomizationResource)
        await writeYamlFile(`${rootDirectory}/projects/default/project.yaml`, defaultAppProjectResource)
        await writeYamlFile(`${rootDirectory}/projects/default/apps/kustomization.yaml`, { resources: [] })
    }

    await writeYamlFile(`${rootDirectory}/root-app.yaml`, rootAppResource)
    await writeYamlFile(`${rootDirectory}/argo-composer.config.yaml`, config)
    await writeYamlFile(`${rootDirectory}/projects/kustomization.yaml`, {
        resources: !addonsAddedInDefaultProject ? ['./default', addons?.path].filter(isNotNil) : [addons.path]
    })
}
