import {
  apply,
  Rule,
  SchematicContext,
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
import select from '@inquirer/select'

const updateKustomization = (projectName: string, appName: string): Rule => {
  return (tree: Tree, _: SchematicContext): Tree => {
    const path = `/projects/${projectName}/apps/kustomization.yaml`
    const file = tree.read(path)?.toString()

    if (!file) {
        throw new SchematicsException('missing file')
    }

    const yaml = parse(file)
    const newProject = `./${appName}.yaml`
    const updatedYaml = {
      ...yaml,
      resources: [
        ...yaml.resources || [],
        newProject
      ]
    }

    tree.overwrite(path, stringify(updatedYaml))

    return tree
  }
}

export const add = (): Rule => {
  return async (tree: Tree) => {
    const projectConfigPath = `./argo-composer.config.yaml`
    const file = tree.read(projectConfigPath)?.toString()

    if (!file) {
      throw new SchematicsException('no initialized project! Please start from init command!')
    }

    const config = parse(file) //todo: add global type for project config and pass whole object
    const mainProjectName = config.name
    const mainRepoURL = config.repoUrl

    const currentProjects = tree.getDir('projects').subdirs

    const appName = await input({ message: 'What name would you like to use for the app?' })
    const imageURL = await input({ message: 'Image URL, example: your-registry.com/your-app' })
    const projectName = await select({
      message: 'Please select project where you want add your app',
      choices: currentProjects.map(currentProject => ({
        name: currentProject.toString(),
        value: currentProject.toString(),
      }))
    }) as string
    const appPort = await input({ message: 'Provide port of the app' })

    const options = {
      appName,
      imageURL,
      projectName,
      appPort
    }

    const templateSource = apply(url('./files'), [
      template({ ...options, ...strings, mainProjectName, mainRepoURL }),
      move(`/projects/${options.projectName}/apps/`)
    ])

    return chain([
      mergeWith(templateSource),
      updateKustomization(options.projectName, options.appName)
    ])
  }
}
