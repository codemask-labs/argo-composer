import { program } from 'commander'
import { runSchematicCommand } from '../runtime'

export const addAppCommand = () =>
    program
        .command('add-app')
        .alias('aa')
        .description('Add new app to project')
        .action(() => {
            runSchematicCommand(':add-app')
        })
