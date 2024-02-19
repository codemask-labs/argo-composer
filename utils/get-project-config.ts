import { parse } from 'yaml'
import { readFileSync } from 'fs-extra'
import { join } from 'node:path'
import { ProjectConfig } from '../types'

export const getProjectConfig = (): ProjectConfig => {
    const projectConfigPath = join(process.cwd(), './argo-composer.config.yaml')
    const file = readFileSync(projectConfigPath).toString()

    if (!file) {
        throw new Error('No project initialized found! Please start from init command!')
    }

    return parse(file)
}
