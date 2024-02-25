import { AppProjectActions, AppProjectActionHandler } from 'lib/utils'

export const createCertManagerAddon: AppProjectActionHandler = async (project: AppProjectActions) => {
    console.log('project:', project)
}
