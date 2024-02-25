import { command } from '@codemaskjs/node-cli-toolkit'

export const COPY_COMMAND = command('copy <resource>', {
    description: 'Copies a resource from project to another project',
    next: () => {
        throw new Error('feature not ready')
    }
})
