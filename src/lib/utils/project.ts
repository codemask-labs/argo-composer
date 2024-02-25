import { parse } from 'yaml'
import { join } from 'node:path'
import { existsSync, readFileSync } from 'node:fs'

export type ProjectConfig = {
    mainProjectName: string
    mainRepositoryUrl: string
    environments: Array<string>
}

const projectConfigPath = join(process.cwd(), './argo-composer.config.yaml')

export const getProjectConfig = (): ProjectConfig => {
    const file = readFileSync(projectConfigPath).toString()

    if (!file) {
        throw new Error('No project initialized found! Please start from init command!')
    }

    return parse(file)
}

export const isProjectExists = () => {
    console.log(projectConfigPath)

    return existsSync(projectConfigPath)
}
