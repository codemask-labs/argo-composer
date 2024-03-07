import { isNotNil } from 'ramda'
import { checkbox, input } from '@inquirer/prompts'
import { isRootDirectoryEmpty, writeYamlFile } from '../utils'
import { appProject } from '../resources'
import { Kustomization } from '../types'

const addAddonApplication = async (rootDirectory: string, addonsProjectName: string, applicationName: string) => {
    await writeYamlFile(`${rootDirectory}/projects/${addonsProjectName}/apps/${applicationName}/application.yaml`, {})
    await writeYamlFile(`${rootDirectory}/projects/${addonsProjectName}/apps/${applicationName}/kustomization.yaml`, {})

    return `./${applicationName}`
}

const addAdditionalApps = async (rootDirectory: string) => {
    const additionalAppChoices = await checkbox({
        message: 'Do you want to install any additional components?',
        choices: [
            { name: 'ingress-nginx', value: 'ingress-nginx' },
            { name: 'cert-manager', value: 'cert-manager' },
            { name: 'reflector', value: 'reflector' },
            { name: 'argocd-image-updater', value: 'argocd-image-updater' }
        ]
    })

    if (!additionalAppChoices.length) {
        return null
    }

    const addonsProjectName = await input({ message: 'What name would you like to use for addons?', default: 'infra' })
    const addonPaths = await Promise.all(additionalAppChoices.map(value => addAddonApplication(rootDirectory, addonsProjectName, value)))

    await writeYamlFile(`${rootDirectory}/projects/${addonsProjectName}/apps/kustomization.yaml`, {
        resources: addonPaths
    })
    await writeYamlFile(`${rootDirectory}/projects/${addonsProjectName}/project.yaml`, {})
    await writeYamlFile(`${rootDirectory}/projects/${addonsProjectName}/kustomization.yaml`, {
        resources: ['./project.yaml', './apps']
    })

    return `./${addonsProjectName}`
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

    const appProjectResource = appProject(rootDirectory, repoURL)
    const addonsProjectPath = await addAdditionalApps(rootDirectory)
    const kustomizationResource: Kustomization = {
        resources: ['./apps', './project.yaml']
    }

    await writeYamlFile(`${rootDirectory}/root-app.yaml`, {})
    await writeYamlFile(`${rootDirectory}/argo-composer.config.yaml`, {
        repoURL,
        environments
    })
    await writeYamlFile(`${rootDirectory}/projects/default/project.yaml`, appProjectResource)
    await writeYamlFile(`${rootDirectory}/projects/default/kustomization.yaml`, kustomizationResource)
    await writeYamlFile(`${rootDirectory}/projects/default/apps/kustomization.yaml`, { resources: [] })
    await writeYamlFile(`${rootDirectory}/projects/kustomization.yaml`, {
        resources: [`./default`, addonsProjectPath].filter(isNotNil)
    })
}
