import { Rule, Tree, chain, SchematicsException } from '@angular-devkit/schematics'
import { parse, stringify } from 'yaml'
import confirm from '@inquirer/confirm'
import select from '@inquirer/select'

const updateKustomization = (projectName: string): Rule => (tree: Tree): Tree => {
    const path = '/projects/kustomization.yaml'
    const file = tree.read(path)?.toString()

    if (!file) {
        throw new SchematicsException('Missing file!')
    }

    const yaml = parse(file)
    const projectDirectory = `./${projectName}`
    const updatedYaml = {
        ...yaml,
        resources: yaml.resources.filter((resource: string) => resource !== projectDirectory)
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
    const currentProjectsCount = currentProjects.length

    if (currentProjectsCount <= 0) {
        throw new SchematicsException('There is no projects inside your repository!')
    }

    const projectName = await select<string>({
        message: 'Which project do you want to remove?',
        choices: currentProjects.map(currentProject => ({
            name: currentProject.toString(),
            value: currentProject.toString()
        }))
    })

    const currentApps = tree.getDir(`projects/${projectName}/apps`).subdirs
    const confirmation = await confirm({ message: `Are you sure that you want to remove project ${projectName} with ${currentApps.length} applications?` })

    if (!confirmation) {
        throw new SchematicsException('Cancelled')
    }

    tree.delete(`projects/${projectName}`)

    return chain([
        updateKustomization(projectName as string)
    ])
}
