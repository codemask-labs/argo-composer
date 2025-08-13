export const ARGO_COMPOSER_DIR = '.argo-composer'
export const DEFAULT_PROFILE = 'default'

export const getProfileDir = (profile: string = DEFAULT_PROFILE) => `${ARGO_COMPOSER_DIR}/${profile}`
export const getTemplatesDir = (profile: string = DEFAULT_PROFILE) => `${getProfileDir(profile)}/templates`
