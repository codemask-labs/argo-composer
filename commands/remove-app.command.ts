import { program } from 'commander'
import { runSchematicCommand } from '../runtime'

export const removeAppCommand = () =>
    program
        .command('remove-app')
        .alias('ra')
        .description('Remove app from project')
        .action(() => {
            runSchematicCommand('schematics:remove-app')
        })
