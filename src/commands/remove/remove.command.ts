import { command } from '@codemaskjs/node-cli-toolkit'

export const REMOVE_COMMAND = command('remove <resource>', {
    short: 'rm',
    description: 'Removes a resource',
    next: () => {
        throw new Error('feature not ready')
    }
})
