import { command } from '@codemaskjs/node-cli-toolkit'

export const APPLY_COMMAND = command('apply <resource>', {
    short: 'cp',
    description: 'Applies changes including `.overrides` to selected resources',
    next: () => {
        throw new Error('feature not ready')
    }
})
