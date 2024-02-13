import { program } from 'commander'
import { runSchematicCommand } from '../runtime'

export const copyAppCommand = () =>
    program
        .command('copy-app')
        .description('Copy app from project to other project')
        .action(() => {
            runSchematicCommand('schematics:copy-app')
        })
