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
  move
} from '@angular-devkit/schematics'
import { parse, stringify } from 'yaml'

type AddAppSchematicOptions = {
  appName: string,
  imageURL: string,
  projectName: string,
  appPort: number
}

const updateKustomization = (projectName: string, appName: string): Rule => {
  return (tree: Tree, _: SchematicContext): Tree => {
    // todo: validate path exists
    const path = `/argocd-project/projects/${projectName}/apps/kustomization.yaml`
    const file = tree.read(path)?.toString()

    if (!file) {
      throw new Error('missing file')
    }

    const yaml = parse(file)
    const newProject = `./${appName}.yaml`
    const updatedYaml = {
      ...yaml,
      resources: [
        ...yaml.resources,
        newProject
      ]
    }

    tree.overwrite(path, stringify(updatedYaml))

    return tree
  }
}

export const add = (options: AddAppSchematicOptions): Rule => {
  return (_tree: Tree, context: SchematicContext) => {
    const templateSource = apply(url('./files'), [
      template({ ...options, ...strings }),
      move(`argocd-project/projects/${options.projectName}/apps/${options.appName}`)
    ])

    const merged = mergeWith(templateSource)

    const rule = chain([
      merged,
      updateKustomization(options.projectName, options.appName)
    ])

    return rule(_tree, context) as Rule
  }
}
