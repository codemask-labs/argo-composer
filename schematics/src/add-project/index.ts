import {
    apply,
    Rule,
    Tree,
    url,
    template,
    strings,
    chain,
    mergeWith,
    move,
    SchematicsException
} from '@angular-devkit/schematics'
import { parse, stringify } from 'yaml'
import { input } from '@inquirer/prompts'

const updateKustomization = (name: string): Rule => (tree: Tree) => {
    const path = '/projects/kustomization.yaml'
    const file = tree.read(path)?.toString()

    if (!file) {
        throw new SchematicsException('missing file')
    }

    const yaml = parse(file)
    const newProjectDirectory = `./${name}`
    const updatedYaml = {
        ...yaml,
        resources: [
            ...yaml.resources || [],
            newProjectDirectory
        ]
    }

    tree.overwrite(path, stringify(updatedYaml))

    return tree
}

export const add = (): Rule => async (tree: Tree) => {
    const projectConfigPath = `./argo-composer.config.yaml`
    const file = tree.read(projectConfigPath)?.toString()

    if (!file) {
        throw new SchematicsException('No project initialized! Please start from init command!')
    }

    const config = parse(file) // todo: add global type for project config and pass whole object/move to lib
    const mainProjectName = config.name
    const mainRepoURL = config.repoUrl
    const projectName = await input({ message: 'What name would you like to use for the project?' })
    const isProjectExists = Boolean(tree.getDir(`projects/${projectName}/`).subfiles.length)

    if (isProjectExists) {
        throw new SchematicsException('project with that name already exists!')
    }

    const templateSource = apply(url('./files'), [
        template({ projectName, ...strings, mainProjectName, mainRepoURL }),
        move('/projects')
    ])

    return chain([
        mergeWith(templateSource),
        updateKustomization(projectName)
    ])
}
