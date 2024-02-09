import { SchematicsException, Tree } from '@angular-devkit/schematics'
import { parse } from 'yaml'
import { ProjectConfig } from './types'

export const getProjectConfig = (tree: Tree): ProjectConfig => {
    const projectConfigPath = `./argo-composer.config.yaml`
    const file = tree.read(projectConfigPath)?.toString()

    if (!file) {
        throw new SchematicsException('no project initialized! Please start from init command!')
    }

    return parse(file)
}
