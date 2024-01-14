import { program } from 'commander'
import { runSchematicCommand } from '../runtime'

export const moveAppCommand = () =>
    program
        .command('move-app')
        .description('Move app from project to other project')
        .action(() => {
            runSchematicCommand('schematics:move-app')
        })
