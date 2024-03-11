import { outputFile, readFileSync, readdirSync, remove } from 'fs-extra'
import { join } from 'node:path'
import { stringify, parse } from 'yaml'
import confirm from '@inquirer/confirm'
import select from '@inquirer/select'
import { StacklessError } from '@codemaskjs/node-cli-toolkit'
import { isProjectExists } from '../utils'
import { Kustomization } from '../types'

export const removeProjectAction = async () => {
    if (!isProjectExists()) {
        throw new StacklessError('No initialized project found! Please start from init command!')
    }

    const currentProjects = readdirSync(join(process.cwd(), 'projects'), { recursive: false, withFileTypes: true })
        .filter(item => item.isDirectory())
        .map(item => item.name)

    const currentProjectsCount = currentProjects.length

    if (currentProjectsCount <= 0) {
        throw new StacklessError('There is no projects inside your repository!')
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

    const confirmation = await confirm({
        message: `Are you sure that you want to remove project ${projectName} with ${currentApps.length} applications?`
    })

    if (!confirmation) {
        throw new StacklessError('Cancelled')
    }

    await remove(join(process.cwd(), 'projects', projectName))

    const currentProjectsKustomizationFile = readFileSync(join(process.cwd(), 'projects', 'kustomization.yaml')).toString()
    const currentProjectsKustomization = parse(currentProjectsKustomizationFile)
    const kustomizationResource: Kustomization = {
        resources: currentProjectsKustomization.resources.filter((projectPath: string) => projectPath !== `./${projectName}`)
    }

    // todo: consider move file operations to utils
    await outputFile(join(process.cwd(), 'projects', 'kustomization.yaml'), stringify(kustomizationResource))
}
