import { dirname, join } from 'node:path'
import { readFileSync, existsSync } from 'node:fs'
import { Package } from './types'

const packageFileName = 'package.json'
const packageRootPathIndex = module.paths.findIndex(directory => {
    const root = dirname(directory)
    const packageRoot = join(root, packageFileName)

    return existsSync(packageRoot)
})

const path = module.paths.at(packageRootPathIndex)!
const root = dirname(path)
const absolutePackageJsonPath = join(root, packageFileName)

export const runtime = {
    root,
    package: JSON.parse(readFileSync(absolutePackageJsonPath, 'utf-8')) as Package
}
