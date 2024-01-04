import { initCommand } from './init.command'
import { addAppCommand } from './add-app.command'
import { addProjectCommand } from './add-project.command'
import { removeAppCommand } from './remove-app.command'
import { removeProjectCommand } from './remove-project.command'

export const commandLoader = () => {
    initCommand()
    addAppCommand()
    addProjectCommand()
    removeAppCommand()
    removeProjectCommand()
}
