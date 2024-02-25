import { outputFile, remove } from 'fs-extra/esm'
import { existsSync, readdirSync } from 'node:fs'
import { readFile } from 'node:fs/promises'
import { join } from 'node:path'
import { parse, stringify } from 'yaml'

export const readYamlFile = async <T>(path: string) => {
    const file = await readFile(join(process.cwd(), path)).catch(() => {
        throw new Error(`File ${path} doesn't exists`)
    })

    return parse(file.toString()) as T
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const writeYamlFile = <T extends Record<string, any> | string>(path: string, content: T) =>
    outputFile(join(process.cwd(), path), typeof content === 'string' ? content : stringify(content)).catch(() => {
        throw new Error(`Error while saving ${path} file!`)
    })

export const isPathExists = (path: string) => existsSync(join(process.cwd(), path))
export const removeFiles = (path: string) => remove(join(process.cwd(), path))

export const getDirectoryList = (path: string) =>
    readdirSync(join(process.cwd(), path), { recursive: false, withFileTypes: true })
        .filter(item => item.isDirectory())
        .map(item => item.name)
