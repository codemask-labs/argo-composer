import { command } from '@codemaskjs/node-cli-toolkit'

export const MOVE_COMMAND = command('move <resource>', {
    short: 'mv',
    description: 'Move resource from project to another project',
    next: () => {
        throw new Error('feature not ready')
    }
})
