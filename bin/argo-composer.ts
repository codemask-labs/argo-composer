#!/usr/bin/env node

import { StacklessError, command, commands, program, usage, version } from '@codemaskjs/node-cli-toolkit'
import { addApplicationAction, addProjectAction, initProjectAction, removeAppAction, removeProjectAction } from '../actions'
import { getComposerPackage } from '../utils'

const { programVersion } = getComposerPackage()

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
            description: 'Adds new resource. Available: `app`, `application` and `project`.',
            next: async ({ resource }) => {
                switch (resource) {
                    case 'app':
                    case 'application':
                        await addApplicationAction()
                        break

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
            description: 'Removes a resource. Available: `app`, `application` and `project`.',
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
    ])
])
