import confirm from '@inquirer/confirm'
import select from '@inquirer/select'
import { StacklessError } from '@codemaskjs/node-cli-toolkit'
import { getDirectoryList, isProjectExists, readYamlFile, removeFiles, writeYamlFile } from '../utils'
import { Kustomization } from '../resources'

export const removeAppAction = async () => {
    if (!isProjectExists()) {
        throw new StacklessError('No initialized project found! Please start from init command!')
    }

    const currentProjects = getDirectoryList('projects')
    const currentProjectsCount = currentProjects.length

    if (currentProjectsCount <= 0) {
        throw new StacklessError('There is no projects inside your repository!')
    }

    const projectName = await select<string>({
        message: 'From which project do you want to remove application from?',
        choices: currentProjects.map(currentProject => ({
            name: currentProject.toString(),
            value: currentProject.toString()
        }))
    })

    const currentApps = getDirectoryList(`projects/${projectName}/apps`)

    if (currentApps.length <= 0) {
        throw new StacklessError('There is no apps inside selected project!')
    }

    const appName = await select<string>({
        message: 'Select app you want to remove',
        choices: currentApps.map(currentApp => ({
            name: currentApp.toString(),
            value: currentApp.toString()
        }))
    })

    const confirmation = await confirm({
        message: `Are you sure that you want to remove '${appName}' application from '${projectName}' project?`
    })

    if (!confirmation) {
        throw new StacklessError('Cancelled')
    }

    await removeFiles(`projects/${projectName}/apps/${appName}`)

    const currentAppsKustomization = await readYamlFile<Kustomization>(`projects/${projectName}/apps/kustomization.yaml`)
    const kustomizationResource: Kustomization = {
        resources: currentAppsKustomization.resources.filter((appPath: string) => appPath !== `./${appName}`)
    }

    await writeYamlFile(`projects/${projectName}/apps/kustomization.yaml`, kustomizationResource)
}
