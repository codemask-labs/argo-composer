import { program } from 'commander'
import { removeProjectAction } from '../actions'

export const removeProjectCommand = () =>
    program.command('remove-project').alias('rp').description('Remove project with applications').action(removeProjectAction)
