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

export const init = () => async (_tree: Tree) => {
    const name = await input({ message: 'What name would you like to use for the project?' })
    const repoURL = await input({ message: 'What is the base URL of GitHub repository?' })
    const environments = await input({ message: 'What will be the environment inside your cluster? Provide separated by `,`', default: 'dev,prod' })
        .then(environments => environments
            .toLowerCase()
            .split(',')
            .map(environment => environment.trim())
        )

    const additionalApps = await checkbox({
        message: 'Do you want to install any additional components?',
        choices: [
            { name: 'ingress-nginx', value: 'ingress-nginx' },
            { name: 'cert-manager', value: 'cert-manager' },
            { name: 'reflector', value: 'reflector' },
            { name: 'argocd-image-updater', value: 'argocd-image-updater' }
        ]
    })

    const addonsProjectName = additionalApps.length
        ? await input({ message: 'What name would you like to use for addons?', default: 'infra' })
        : undefined

    const addonsProject = additionalApps.length
        ? [
            mergeWith(
                apply(url('./addons/addons-project'), [
                    template({ repoURL, name, addonsProjectName, ...strings })
                ])
            )
        ]
        : []

    const addons = addonsProjectName
        ? additionalApps.map(addon => addonTemplate(addon, { name, repoURL }, addonsProjectName))
        : []

    const templateSource = apply(url('./files'), [
        template({ name, repoURL, environments, ...strings })
    ])

    return chain([
        mergeWith(templateSource),
        ...addonsProject,
        ...addons,
        updateAddonsKustomization(name, additionalApps, addonsProjectName),
        updateProjectKustomization(name, addonsProjectName)
    ])
}
