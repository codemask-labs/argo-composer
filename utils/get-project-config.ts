import { parse } from 'yaml'
import { existsSync, readFileSync } from 'fs-extra'
import { join } from 'node:path'
import { StacklessError } from '@codemaskjs/node-cli-toolkit'
import { ProjectConfig } from '../types'
import { ARGO_COMPOSER_DIR, DEFAULT_PROFILE } from '../constants/paths'

// v2+: configuration moved under a dedicated `.argo-composer` directory
const mergedConfigPath = join(process.cwd(), `${ARGO_COMPOSER_DIR}/argo-composer.yaml`)
const profilePath = join(process.cwd(), `${ARGO_COMPOSER_DIR}/${DEFAULT_PROFILE}/profile.yaml`)

export const getProjectConfig = (): ProjectConfig => {
    if (!existsSync(mergedConfigPath)) {
        throw new StacklessError(`Argo Composer cannot find '.argo-composer/argo-composer.yaml'. Was the root directory initialized properly?`)
    }

    const mergedRaw = readFileSync(mergedConfigPath).toString()
    if (mergedRaw.trim().length === 0) {
        throw new Error('No project initialized found! Please start from init command!')
    }

    const merged = parse(mergedRaw) as { mainRepositoryUrl?: string; environments?: Array<string> }

    // Prefer environments from profile.yaml; fallback to merged config (for backward compat) or ['dev']
    const environments: Array<string> = (() => {
        if (existsSync(profilePath)) {
            try {
                const prof = parse(readFileSync(profilePath).toString()) as { name?: string; environments?: Array<string> }
                if (Array.isArray(prof.environments) && prof.environments.length > 0) {
                    return prof.environments
                }
            } catch {
                // ignore and continue to fallbacks
            }
        }

        if (Array.isArray(merged.environments) && merged.environments.length > 0) {
            return merged.environments
        }

        return ['dev']
    })()

    return {
        mainRepositoryUrl: merged.mainRepositoryUrl ?? '',
        environments,
    }
}

export const isProjectExists = () => existsSync(mergedConfigPath)
