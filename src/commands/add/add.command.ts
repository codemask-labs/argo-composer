import { command } from '@codemaskjs/node-cli-toolkit'
import { addApplication, addProject } from './actions'

export const ADD_COMMAND = command('add <resource>', {
    description: 'Adds new resource',
    next: params => {
        switch (params.resource) {
            case 'app':
            case 'application':
                return addApplication()

            case 'project':
                return addProject()

            default:
                throw new Error(
                    `Failed to add resource name '${params.resource}' is not known to argo-composer. The available resources are: application, app, project`
                )
        }
    }
})
