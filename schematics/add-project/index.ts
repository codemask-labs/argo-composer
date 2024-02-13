import { apply, Rule, Tree, url, template, strings, chain, mergeWith, move, SchematicsException } from '@angular-devkit/schematics'
import { parse, stringify } from 'yaml'
import { input } from '@inquirer/prompts'
import { getProjectConfig } from '../common'

const updateKustomization =
    (name: string): Rule =>
    (tree: Tree) => {
        const path = '/projects/kustomization.yaml'
        const file = tree.read(path)?.toString()

        if (!file) {
            throw new SchematicsException('Missing file!')
        }

        const yaml = parse(file)
        const newProjectDirectory = `./${name}`
        const updatedYaml = {
            ...yaml,
            resources: [...(yaml.resources || []), newProjectDirectory]
        }

        tree.overwrite(path, stringify(updatedYaml))

        return tree
    }

export const add = (): Rule => async (tree: Tree) => {
    const { name: mainProjectName, repoUrl: mainRepoURL } = getProjectConfig(tree)

    const projectName = await input({ message: 'What name would you like to use for the project?' })
    const isProjectExists = Boolean(tree.getDir(`projects/${projectName}/`).subfiles.length)

    if (isProjectExists) {
        throw new SchematicsException('Project with that name already exists!')
    }

    const templateSource = apply(url('./files'), [template({ projectName, ...strings, mainProjectName, mainRepoURL }), move('/projects')])

    return chain([mergeWith(templateSource), updateKustomization(projectName)])
}
