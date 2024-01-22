import {
    apply,
    Tree,
    url,
    template,
    strings,
    chain,
    mergeWith,
    Rule,
    SchematicsException
} from '@angular-devkit/schematics'
import checkbox from '@inquirer/checkbox'
import { parse, stringify } from 'yaml'
import { input } from '@inquirer/prompts'

interface InitProjectSchematicOptions {
    name: string;
    repoURL: string
}

const updateProjectKustomization = (projectName: string, name?: string): Rule => (tree: Tree): Tree => {
    if (!name) {
        return tree
    }

    const path = `${projectName}/projects/kustomization.yaml`
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

const updateAddonsKustomization = (projectName: string, addons: Array<string>, addonsProjectName?: string): Rule => (tree: Tree): Tree => {
    if (addons.length === 0 || !addonsProjectName) {
        return tree
    }

    const path = `${projectName}/projects/${addonsProjectName}/apps/kustomization.yaml`

    const newApplicationDirectories = addons.map(addon => `./${addon}`)
    const updatedYaml = {
        resources: [
            ...newApplicationDirectories
        ]
    }

    tree.overwrite(path, stringify(updatedYaml))

    return tree
}

export const addonTemplate = (addon: string, options: InitProjectSchematicOptions, addonsProjectName: string) => mergeWith(
    apply(url(`./addons/${addon}`), [
            template({ ...options, ...strings, addonsProjectName })
        ]
    )
)

export const init = (options: InitProjectSchematicOptions) => async (_tree: Tree) => {
    const additionalApps = await checkbox({
        message: 'Do you want to install any additional components?',
        choices: [
            { name: 'ingress-nginx', value: 'ingress-nginx' },
            { name: 'cert-manager', value: 'cert-manager' },
            { name: 'reflector', value: 'reflector' },
            { name: 'argocd-image-updater', value: 'argocd-image-updater' }
        ]
    }) as Array<string>

    const addonsProjectName = additionalApps.length
        ? await input({ message: 'What name would you like to use for addons?', default: 'infra' })
        : undefined

    const addonsProject = additionalApps.length
        ? [
            mergeWith(
                apply(url('./addons/addons-project'), [
                    template({ ...options, addonsProjectName, ...strings })
                ])
            )
        ]
        : []

    const addons = addonsProjectName
        ? additionalApps.map(addon => addonTemplate(addon, options, addonsProjectName))
        : []

    const templateSource = apply(url('./files'), [
        template({ ...options, ...strings })
    ])

    return chain([
        mergeWith(templateSource),
        ...addonsProject,
        ...addons,
        updateAddonsKustomization(options.name, additionalApps, addonsProjectName),
        updateProjectKustomization(options.name, addonsProjectName)
    ])
}
