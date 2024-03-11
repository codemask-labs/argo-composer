import { program } from 'commander'
import { addApplicationAction } from '../actions'

export const addAppCommand = () => program.command('add-app').alias('aa').description('Add new app to project').action(addApplicationAction)
