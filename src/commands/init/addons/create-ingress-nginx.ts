import { AppProjectActionHandler, AppProjectActions } from 'lib/utils'

export const createIngressNginxAddon: AppProjectActionHandler = async (project: AppProjectActions) => {
    console.log('project:', project)
}
