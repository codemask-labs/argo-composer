import { existsSync, outputFile, readdirSync, readdir, readFile, remove } from 'fs-extra'
import { statSync } from 'node:fs'
import { join } from 'node:path'
import { parse, stringify } from 'yaml'

/**
 * A list of whitelisted directories and/or files to conisder
 */
const ROOT_DIRECTORY_WHITELIST = [
    '.git'
]

export const readYamlFile = async <T>(path: string) => {
    const file = await readFile(join(process.cwd(), path)).catch(() => {
        throw new Error(`File ${path} doesn't exists`)
    })

    return parse(file.toString()) as T
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const writeYamlFile = (path: string, content: Record<any, any> | string) => outputFile(join(process.cwd(), path), typeof content === 'string' ? content : stringify(content)).catch(() => {
    throw new Error(`Error while saving ${path} file!`)
})

export const isPathExists = (path: string) => existsSync(join(process.cwd(), path))
export const removeFiles = (path: string) => remove(join(process.cwd(), path))

export const getDirectoryList = (path: string) =>
    readdirSync(join(process.cwd(), path), { recursive: false, withFileTypes: true })
        .filter(item => item.isDirectory())
        .map(item => item.name)

export const isDirectory = (path: string) =>
    statSync(join(process.cwd(), path)).isDirectory()

export const isRootDirectoryEmpty = (path: string) => readdir(join(process.cwd(), path))
    .then(files => files.every(file => ROOT_DIRECTORY_WHITELIST.includes(file)))
    .catch(() => true)
