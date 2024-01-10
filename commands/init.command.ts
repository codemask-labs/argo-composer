import { program } from 'commander'
import { runSchematicCommand } from '../runtime'

export const initCommand = () =>
    program
        .command('init-project')
        .alias('i')
        .description('Init new project')
        .action(() => {
            runSchematicCommand('schematics:init-project')
        })
