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
import select from '@inquirer/select'
import confirm from '@inquirer/confirm'
import { getProjectConfig } from '../common'

type Options = {
    appName: string,
    imageURL: string,
    projectName: string,
    appPort: string,
    useImageAutoUpdater: boolean,
    environments: Array<string>,
    shouldAppContainOverlays: boolean
}

const updateKustomization = (projectName: string, appName: string): Rule => (tree: Tree): Tree => {
    const path = `/projects/${projectName}/apps/kustomization.yaml`
    const file = tree.read(path)?.toString()

    if (!file) {
        throw new SchematicsException('missing file')
    }

    const yaml = parse(file)
    const newApplicationDirectory = `./${appName}`
    const updatedYaml = {
        ...yaml,
        resources: [
            ...yaml.resources || [],
            newApplicationDirectory
        ]
    }

    tree.overwrite(path, stringify(updatedYaml))

    return tree
}

const envTemplate = (envName: string, options: Options) => mergeWith(
    apply(url(`./overlay`), [
            template({ envName, ...options, ...strings }),
            move(`/projects/${options.projectName}/apps/${options.appName}/overlays/${envName}`)
        ]
    )
)

const overlayBaseTemplate = (options: Options) => mergeWith(
    apply(url(`./overlay-base`), [
            template({ ...options, ...strings }),
            move(`/projects/${options.projectName}/apps/${options.appName}/base`)
        ]
    )
)

const addResources = (options: Options) => mergeWith(
    apply(url(`./resources`), [
            template({ ...options, ...strings }),
            move(`/projects/${options.projectName}/apps/${options.appName}/resources`)
        ]
    )
)

export const add = (): Rule => async (tree: Tree) => {
    const { name: mainProjectName, environments, repoUrl: mainRepoURL } = getProjectConfig(tree)

    const currentProjects = tree.getDir('projects').subdirs
    const shouldAppContainOverlays = await confirm({ message: 'Use overlays (multiple envs)?', default: true })
    const appName = await input({ message: 'What name would you like to use for the app?' })
    const imageURL = await input({ message: 'Image URL, example: your-registry.com/your-app' })
    const projectName = await select({
        message: 'Please select project where you want add your app',
        choices: currentProjects.map(currentProject => ({
            name: currentProject.toString(),
            value: currentProject.toString()
        }))
    })
    const appPort = await input({ message: 'Provide port of the app' })
    const useImageAutoUpdater = await confirm({ message: 'Use image auto-updater?', default: true })

    const options: Options = {
        appName,
        imageURL,
        projectName,
        appPort,
        useImageAutoUpdater,
        environments,
        shouldAppContainOverlays
    }

    const overlays = shouldAppContainOverlays
        ? environments.map(env => envTemplate(env, options))
        : []

    const base = overlays.length > 0 && shouldAppContainOverlays
        ? [overlayBaseTemplate(options)]
        : []

    const resources = !shouldAppContainOverlays
        ? [addResources(options)]
        : []

    const templateSource = apply(url('./files'), [
        template({ ...options, ...strings, mainProjectName, mainRepoURL }),
        move(`/projects/${options.projectName}/apps/`)
    ])

    return chain([
        mergeWith(templateSource),
        ...overlays,
        ...base,
        ...resources,
        updateKustomization(options.projectName, options.appName)
    ])
}
