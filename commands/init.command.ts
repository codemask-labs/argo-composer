import { program } from 'commander'
import { initProjectAction } from '../actions'

export const initCommand = () =>
    program
        .command('init-project')
        .alias('i')
        .description('Init new project')
        .action(initProjectAction)
