import { AppProjectActionHandler, AppProjectActions } from 'lib/utils'

export const createReflectorAddong: AppProjectActionHandler = async (project: AppProjectActions) => {
    console.log('project:', project)
}
