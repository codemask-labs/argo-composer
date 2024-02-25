import { command } from '@codemaskjs/node-cli-toolkit'

export const RENAME_COMMAND = command('rename <resource>', {
    description: 'Renames a resource',
    next: () => {
        throw new Error('feature not ready')
    }
})
