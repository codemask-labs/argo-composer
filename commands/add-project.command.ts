import { program } from 'commander'
import { runSchematicCommand } from '../runtime'

export const addProjectCommand = () =>
    program
        .command('add-project')
        .alias('ap')
        .description('Add new project')
        .action(() => {
            runSchematicCommand(':add-project')
        })
