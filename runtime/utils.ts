import { join } from 'path'
import { spawnSync } from 'node:child_process'
import { runtime } from './constants'

export const resolve = (path: string) => join(runtime.root, path)

export const runSchematicCommand = (command: string) => {
    const args = [
        resolve('node_modules/@angular-devkit/schematics-cli/bin/schematics.js'),
        resolve(command)
    ]

    const result = spawnSync('node', args, {
        stdio: 'inherit',
        cwd: process.cwd()
    })

    process.exit(result.error ? 1 : 0)
}
