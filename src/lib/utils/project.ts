import { join } from 'node:path'
import { existsSync, readFileSync } from 'node:fs'
import { parse } from 'yaml'
import { PROJECT_CONFIG_NAME } from 'lib/common'

export type ProjectConfig = {
    repoURL: string
    environments: Array<string>
}

const projectConfigPath = join(process.cwd(), './argo-composer.config.yaml')

export const getProjectConfig = (path?: string): ProjectConfig => {
    const root = path ? join(path, PROJECT_CONFIG_NAME) : join(process.cwd(), PROJECT_CONFIG_NAME)
    const file = readFileSync(root).toString()

    if (!file) {
        throw new Error('No project initialized found! Please start from init command!')
    }

    return parse(file)
}

export const isProjectExists = () => {
    console.log(projectConfigPath)

    return existsSync(projectConfigPath)
}
