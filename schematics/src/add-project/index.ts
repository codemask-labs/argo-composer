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

interface AddProjectSchematicOptions {
  name: string
}

function updateKustomization(name: string): Rule {
  return (tree: Tree, _: SchematicContext): Tree => {
    // todo: validate path exists
    const path = `/argocd-project/projects/kustomization.yaml`
    const file = tree.read(path)?.toString()

    if (!file) {
      throw new Error('missing file')
    }

    const yaml = parse(file)
    const newProject = `./${name}.yaml`
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

export function add(options: AddProjectSchematicOptions): Rule {
  return (_tree: Tree, context: SchematicContext) => {
    const templateSource = apply(url('./files'), [
      template({ ...options, ...strings }),
      move('argocd-project/projects')
    ])

    const merged = mergeWith(templateSource)

    const rule = chain([
      merged,
      updateKustomization(options.name)
    ])

    return rule(_tree, context) as Rule
  }
}
