import { outputFile, readFileSync, readdirSync, remove } from 'fs-extra'
import { join } from 'node:path'
import { stringify, parse } from 'yaml'
import confirm from '@inquirer/confirm'
import select from '@inquirer/select'
import { isProjectExists } from '../utils'

export const removeAppAction = async () => {
    if (isProjectExists()) {
        throw new Error('No initialized project found! Please start from init command!')
    }

    const currentProjects = readdirSync(join(process.cwd(), 'projects'), { recursive: false, withFileTypes: true })
        .filter(item => item.isDirectory())
        .map(item => item.name)

    const currentProjectsCount = currentProjects.length

    if (currentProjectsCount <= 0) {
        throw new Error('There is no projects inside your repository!')
    }

    const projectName = await select<string>({
        message: 'Which project do you want to remove?',
        choices: currentProjects.map(currentProject => ({
            name: currentProject.toString(),
            value: currentProject.toString()
        }))
    })

    const currentApps = readdirSync(join(process.cwd(), 'projects', projectName, 'apps'), { recursive: false, withFileTypes: true })
        .filter(item => item.isDirectory())
        .map(item => item.name)

    if (currentApps.length <= 0) {
        throw new Error('There is no apps inside selected project!')
    }

    const appName = await select<string>({
        message: 'Select app you want to remove',
        choices: currentApps.map(currentApp => ({
            name: currentApp.toString(),
            value: currentApp.toString()
        }))
    })

    const confirmation = await confirm({ message: `Are you sure that you want to remove app ${projectName}/apps/${appName}?` })

    if (!confirmation) {
        throw new Error('Cancelled')
    }

    await remove(join(process.cwd(), 'projects', projectName, 'apps', appName))

    const currentAppsKustomizationFile = readFileSync(join(process.cwd(), 'projects', projectName, 'apps', 'kustomization.yaml')).toString()
    const currentAppsKustomization = parse(currentAppsKustomizationFile)

    // todo: Provide type
    const kustomizationResource = {
        resources: currentAppsKustomization.resources.filter((appPath: string) => appPath === `./${appName}`)
    }

    // todo: consider move file operations to utils
    await outputFile(join(process.cwd(), 'projects', projectName, 'apps', 'kustomization.yaml'), stringify(kustomizationResource))
}
