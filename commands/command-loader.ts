import { initCommand } from './init.command'
import { addApplicationCommand } from './add-application.command'
import { addProjectCommand } from './add-project.command'
import { removeAppCommand } from './remove-app.command'
import { removeProjectCommand } from './remove-project.command'

export const commandLoader = () => {
    initCommand()
    addApplicationCommand()
    addProjectCommand()
    removeAppCommand()
    removeProjectCommand()
}
