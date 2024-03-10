import { StacklessError, command, commands, getPackageJson, program, usage, version } from '@codemaskjs/node-cli-toolkit'
import { addProjectAction, initProjectAction, removeAppAction, removeProjectAction } from '../actions'

const { version: programVersion } = getPackageJson()

program([
    version(programVersion),
    usage('argo-composer'),
    commands([
        command('init', {
            short: 'i',
            description: 'Initializes argo composer root directory',
            next: () => initProjectAction()
        }),
        command('add <resource>', {
            description: 'Adds new resource',
            next: async ({ resource }) => {
                switch (resource) {
                    case 'app':
                    case 'application':
                        throw new Error('not yet supported')

                    case 'project':
                        await addProjectAction()
                        break

                    default:
                        throw new StacklessError(`Adding resource '${resource}' is not supported. Allowed: 'app', 'application' or 'project'.`)
                }
            }
        }),
        command('remove <resource>', {
            short: 'rm',
            description: 'Removes a resource',
            next: async ({ resource }) => {
                switch (resource) {
                    case 'app':
                    case 'application':
                        await removeAppAction()
                        break

                    case 'project':
                        await removeProjectAction()
                        break

                    default:
                        throw new StacklessError(`Removing resource '${resource}' is not supported. Allowed: 'app', 'application' or 'project'.`)
                }
            }
        })
        // todo: implement the below commands
        // command('move <resource>', {
        //     short: 'mv',
        //     description: 'Moves a resource from project to another project',
        //     next: params => {}
        // }),
        // command('copy <resource>', {
        //     short: 'cp',
        //     description: 'Copies a resource from project to another project',
        //     next: params => {}
        // }),
        // command('rename <resource>', {
        //     description: 'Renames a resource',
        //     next: params => {
        //         console.log('Hello debug!', params)
        //     }
        // }),
        // command('apply <resource>', {
        //     description: 'Applies changes including `.overrides` to selected resources',
        //     next: params => {}
        // })
    ])
])
