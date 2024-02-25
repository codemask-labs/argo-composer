import { join, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

export const PROJECT_CONFIG_NAME = 'argo-composer.config.yaml'
export const ROOT_PROJECTS_DIR = join(process.cwd(), 'projects')
export const ROOT_PROJECT_CONFIG_PATH = join(process.cwd(), PROJECT_CONFIG_NAME)
export const ROOT_COMPOSER_DIR = resolve(fileURLToPath(import.meta.url), '..', '..')
