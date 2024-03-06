import { input } from '@inquirer/prompts'
import { getProjectConfig, isPathExists, readYamlFile, writeYamlFile } from '../utils'
import { appProject } from '../resources'
import { Kustomization } from '../types'
import checkbox from '@inquirer/checkbox/dist/esm/types'

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

    const additionalApps = await checkbox({
        message: 'Do you want to install any additional components?',
        choices: [
            { name: 'ingress-nginx', value: 'ingress-nginx' },
            { name: 'cert-manager', value: 'cert-manager' },
            { name: 'reflector', value: 'reflector' },
            { name: 'argocd-image-updater', value: 'argocd-image-updater' }
        ]
    })

    const addonsProjectName = additionalApps.length
        ? await input({ message: 'What name would you like to use for addons?', default: 'infra' })
        : undefined

    const currentProjectsKustomization = await readYamlFile<Kustomization>('projects/kustomization.yaml')
    const appProjectResource = appProject(projectName, mainRepositoryUrl)
    const kustomizationResource: Kustomization = {
        resources: ['./apps', './project.yaml']
    }

    await writeYamlFile(`projects/${projectName}/project.yaml`, appProjectResource)
    await writeYamlFile(`projects/${projectName}/kustomization.yaml`, kustomizationResource)
    await writeYamlFile(`projects/${projectName}/apps/kustomization.yaml`, { resources: [] })
    await writeYamlFile(`projects/kustomization.yaml`, {
        resources: [...(currentProjectsKustomization?.resources ?? []), `./${projectName}`]
    })
}
