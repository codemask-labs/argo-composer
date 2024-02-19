import { program } from 'commander'
import { removeAppAction } from '../actions'

export const removeAppCommand = () =>
    program
        .command('remove-app')
        .alias('ra')
        .description('Remove app from project')
        .action(removeAppAction)
