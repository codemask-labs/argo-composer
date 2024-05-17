import { existsSync } from 'node:fs'
import { join, parse } from 'node:path'
import { StacklessError, getPackageJson } from '@codemaskjs/node-cli-toolkit'

export const getComposerPackage = () => {
    const absolutePackageJsonPath = module.paths.reduce<string | null>((result, root) => {
        const { dir: directory } = parse(root)
        const path = join(directory, 'package.json')

        if (result || !existsSync(path)) {
            return result
        }

        return directory
    }, null)

    if (!absolutePackageJsonPath) {
        throw new StacklessError(`Failed to find argo-composer package.json`)
    }

    const values = getPackageJson(absolutePackageJsonPath)

    return {
        programVersion: values.version
    }
}
