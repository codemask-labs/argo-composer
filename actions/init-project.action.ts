import { isNotNil } from 'ramda'
import { checkbox, input } from '@inquirer/prompts'
import { isDirectoryEmpty, writeYamlFile } from '../utils'
import { appProject } from '../resources'
import { Kustomization } from '../types'

const addAddonApplication = async (root: string, addonsProjectName: string, applicationName: string) => {
    await writeYamlFile(`${root}/projects/${addonsProjectName}/apps/${applicationName}/application.yaml`, {})
    await writeYamlFile(`${root}/projects/${addonsProjectName}/apps/${applicationName}/kustomization.yaml`, {})

    return `./${applicationName}`
}

const addAdditionalApps = async (root: string) => {
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
    const addonPaths = await Promise.all(additionalAppChoices.map(value => addAddonApplication(root, addonsProjectName, value)))

    await writeYamlFile(`${root}/projects/${addonsProjectName}/apps/kustomization.yaml`, {
        resources: addonPaths
    })
    await writeYamlFile(`${root}/projects/${addonsProjectName}/project.yaml`, {})
    await writeYamlFile(`${root}/projects/${addonsProjectName}/kustomization.yaml`, {
        resources: ['./project.yaml', './apps']
    })

    return `./${addonsProjectName}`
}

export const initProjectAction = async () => {
    const root = await input({ message: 'What root directory would you like to use for the project?', default: '.' })

    if (!(await isDirectoryEmpty(root))) {
        throw new Error(`Directory '${root}' is not empty`)
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

    const appProjectResource = appProject(root, repoURL)
    const addons = await addAdditionalApps(root)
    const kustomizationResource: Kustomization = {
        resources: ['./apps', './project.yaml']
    }

    await writeYamlFile(`${root}/root-app.yaml`, {})
    await writeYamlFile(`${root}/argo-composer.config.yaml`, {
        repoURL,
        environments
    })
    await writeYamlFile(`${root}/projects/default/project.yaml`, appProjectResource)
    await writeYamlFile(`${root}/projects/default/kustomization.yaml`, kustomizationResource)
    await writeYamlFile(`${root}/projects/default/apps/kustomization.yaml`, { resources: [] })
    await writeYamlFile(`${root}/projects/kustomization.yaml`, {
        resources: [`./default`, addons].filter(isNotNil)
    })
}
