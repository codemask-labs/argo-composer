import { parse } from 'yaml'
import { existsSync, readFileSync } from 'fs-extra'
import { join } from 'node:path'
import { ProjectConfig } from '../types'
import { StacklessError } from '@codemaskjs/node-cli-toolkit'

const projectConfigPath = join(process.cwd(), './argo-composer.config.yaml')

export const getProjectConfig = (): ProjectConfig => {
    if (!existsSync(projectConfigPath)) {
        throw new StacklessError(`Argo Composer cannot 'find argo-composer.config.yaml'. Was the root directory initialized properly?`)
    }

    const file = readFileSync(projectConfigPath).toString()

    if (!file) {
        throw new Error('No project initialized found! Please start from init command!')
    }

    return parse(file)
}

export const isProjectExists = () => existsSync(projectConfigPath)
