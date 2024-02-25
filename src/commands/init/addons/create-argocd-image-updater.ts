import { AppProjectActions, AppProjectActionHandler } from 'lib/utils'

export const createArgocdImageUpdaterAddon: AppProjectActionHandler = async (project: AppProjectActions) => {
    console.log('project:', project)
}
