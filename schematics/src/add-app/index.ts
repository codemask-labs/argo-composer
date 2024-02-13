import { parse, stringify } from 'yaml'
import { apply, Rule, Tree, url, template, strings, chain, mergeWith, move, SchematicsException, filter } from '@angular-devkit/schematics'
import { input } from '@inquirer/prompts'
import select from '@inquirer/select'
import confirm from '@inquirer/confirm'
import { getProjectConfig } from '../common'

type Options = {
    appName: string
    imageURL: string
    projectName: string
    appPort: string
    useImageAutoUpdater: boolean
    environments: Array<string>
    shouldAppContainOverlays: boolean
    useHorizontalPodAutoscaler: boolean
}

const updateKustomization =
    (projectName: string, appName: string): Rule =>
    (tree: Tree): Tree => {
        const path = `/projects/${projectName}/apps/kustomization.yaml`
        const file = tree.read(path)?.toString()

        if (!file) {
            throw new SchematicsException('Missing file!')
        }

        const yaml = parse(file)
        const newApplicationDirectory = `./${appName}`
        const updatedYaml = {
            ...yaml,
            resources: [...(yaml.resources || []), newApplicationDirectory]
        }

        tree.overwrite(path, stringify(updatedYaml))

        return tree
    }

const addOverlay = (name: string, options: Options) => {
    const variables = { ...strings, ...options, envName: name }
    const destination = `/projects/${options.projectName}/apps/${options.appName}/overlays/${name}`
    const source = url('./overlay')

    return mergeWith(apply(source, [template(variables), move(destination)]))
}

const addOverlayBase = (options: Options) => {
    const destination = `/projects/${options.projectName}/apps/${options.appName}/base`
    const variables = { ...strings, ...options }
    const source = url('./overlay-base')

    return mergeWith(
        apply(source, [
            filter(path => {
                if (!options.useHorizontalPodAutoscaler && path.endsWith('hpa.yaml')) {
                    return false
                }

                return true
            }),
            template(variables),
            move(destination)
        ])
    )
}

const addResources = (options: Options) =>
    mergeWith(
        apply(url(`./resources`), [template({ ...options, ...strings }), move(`/projects/${options.projectName}/apps/${options.appName}/resources`)])
    )

export const add = (): Rule => async (tree: Tree) => {
    const { name: mainProjectName, environments, repoUrl: mainRepoURL } = getProjectConfig(tree)

    const currentProjects = tree.getDir('projects').subdirs
    const shouldAppContainOverlays = await confirm({
        message: 'Use overlays (multiple envs)?',
        default: true
    })
    const appName = await input({
        message: 'What name would you like to use for the app?'
    })
    const imageURL = await input({
        message: 'Image URL, example: your-registry.com/your-app'
    })
    const projectName = await select({
        message: 'Please select project where you want add your app',
        choices: currentProjects.map(currentProject => ({
            name: currentProject.toString(),
            value: currentProject.toString()
        }))
    })
    const appPort = await input({ message: 'Provide port of the app' })
    const useImageAutoUpdater = await confirm({
        message: 'Use image auto-updater?',
        default: true
    })
    const useHorizontalPodAutoscaler = await confirm({
        message: 'Use HPA (Horizontal Pod Autoscaler)?',
        default: true
    })

    const options: Options = {
        appName,
        imageURL,
        projectName,
        appPort,
        useImageAutoUpdater,
        environments,
        shouldAppContainOverlays,
        useHorizontalPodAutoscaler
    }

    const base = shouldAppContainOverlays ? [addOverlayBase(options)] : []
    const overlays = shouldAppContainOverlays ? environments.map(env => addOverlay(env, options)) : []
    const resources = !shouldAppContainOverlays ? [addResources(options)] : []
    const templateSource = apply(url('./files'), [
        template({ ...options, ...strings, mainProjectName, mainRepoURL }),
        move(`/projects/${options.projectName}/apps/`)
    ])

    return chain([
        mergeWith(templateSource),
        updateKustomization(options.projectName, options.appName),
        ...base,
        ...overlays,
        ...resources
    ])
}
