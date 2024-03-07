import { isNotNil } from 'ramda'
import { checkbox, input } from '@inquirer/prompts'
import { writeYamlFile } from '../utils'
import { appProject } from '../resources'
import { Kustomization } from '../types'

const addAddonApplication = async (addonsProjectName: string, applicationName: string) => {
    await writeYamlFile(`projects/${addonsProjectName}/apps/${applicationName}/application.yaml`, {})
    await writeYamlFile(`projects/${addonsProjectName}/apps/${applicationName}/kustomization.yaml`, {})

    return `./${applicationName}`
}

const addAdditionalApps = async () => {
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
    const addonPaths = await Promise.all(additionalAppChoices.map(value => addAddonApplication(addonsProjectName, value)))

    await writeYamlFile(`projects/${addonsProjectName}/apps/kustomization.yaml`, {
        resources: addonPaths
    })
    await writeYamlFile(`projects/${addonsProjectName}/project.yaml`, {})
    await writeYamlFile(`projects/${addonsProjectName}/kustomization.yaml`, {
        resources: ['./project.yaml', './apps']
    })

    return `./${addonsProjectName}`
}

export const initProjectAction = async () => {
    const name = await input({ message: 'What name would you like to use for the project?' })
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

    const appProjectResource = appProject(name, repoURL)
    const additionalAppsPath = await addAdditionalApps()
    const kustomizationResource: Kustomization = {
        resources: ['./apps', './project.yaml']
    }

    await writeYamlFile(`argo-composer.config.yaml`, {
        name,
        repoURL,
        environments
    })
    await writeYamlFile(`projects/default/project.yaml`, appProjectResource)
    await writeYamlFile(`projects/default/kustomization.yaml`, kustomizationResource)
    await writeYamlFile(`projects/default/apps/kustomization.yaml`, { resources: [] })
    await writeYamlFile(`projects/kustomization.yaml`, {
        resources: [`./default`, additionalAppsPath].filter(isNotNil)
    })
}
