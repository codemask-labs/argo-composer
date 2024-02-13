import { Rule, Tree, chain, SchematicsException } from '@angular-devkit/schematics'
import { parse, stringify } from 'yaml'
import confirm from '@inquirer/confirm'
import select from '@inquirer/select'

const updateKustomization =
    (projectName: string, appName: string): Rule =>
    (tree: Tree): Tree => {
        const path = `/projects/${projectName}/apps/kustomization.yaml`
        const file = tree.read(path)?.toString()

        if (!file) {
            throw new SchematicsException('Missing file!')
        }

        const yaml = parse(file)
        const applicationDirectory = `./${appName}`
        const updatedYaml = {
            ...yaml,
            resources: yaml.resources.filter((resource: string) => resource !== applicationDirectory)
        }

        tree.overwrite(path, stringify(updatedYaml))

        return tree
    }

export const remove = (): Rule => async (tree: Tree) => {
    const projectConfigPath = `./argo-composer.config.yaml`
    const file = tree.read(projectConfigPath)?.toString()

    if (!file) {
        throw new SchematicsException('No initialized project found! Please start from init command!')
    }

    const currentProjects = tree.getDir('projects').subdirs

    if (currentProjects.length <= 0) {
        throw new SchematicsException('There is no projects inside your repository!')
    }

    const projectName = await select<string>({
        message: 'From which project do you want to remove app?',
        choices: currentProjects.map(currentProject => ({
            name: currentProject.toString(),
            value: currentProject.toString()
        }))
    })

    const currentApps = tree.getDir(`projects/${projectName}/apps`).subdirs

    if (currentProjects.length <= 0) {
        throw new SchematicsException('There is no apps inside selected project!')
    }

    const appName = await select<string>({
        message: 'Select app you want to remove',
        choices: currentApps.map(currentApp => ({
            name: currentApp.toString(),
            value: currentApp.toString()
        }))
    })

    const confirmation = await confirm({ message: `Are you sure that you want to remove app ${projectName}/apps/${appName}?` })

    if (!confirmation) {
        throw new SchematicsException('Cancelled')
    }

    tree.delete(`projects/${projectName}/apps/${appName}`)

    return chain([updateKustomization(projectName, appName)])
}
