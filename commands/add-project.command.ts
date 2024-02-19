import { program } from 'commander'
import { addProjectAction } from '../actions'

export const addProjectCommand = () => program.command('add-project').alias('ap').description('Add new project').action(addProjectAction)
