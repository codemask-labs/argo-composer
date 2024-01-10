import { program } from 'commander'
import { runSchematicCommand } from '../runtime'

export const removeProjectCommand = () =>
    program
        .command('remove-project')
        .alias('rp')
        .description('Remove project with applications')
        .action(() => {
            runSchematicCommand('schematics:remove-project')
        })
