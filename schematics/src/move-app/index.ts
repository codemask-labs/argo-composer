import { parse, stringify } from 'yaml'
import {
    Rule,
    SchematicsException,
    Tree,
    chain,
    move
} from '@angular-devkit/schematics'
import { select } from '@inquirer/prompts'

type YamlResource = {
    path: string
    data: Record<string, any>
}

const getYamlResources = (tree: Tree, pathOrDirectory: string): Array<YamlResource> => {
    const file = tree.exists(pathOrDirectory) && tree.get(pathOrDirectory)
    const directory = !tree.exists(pathOrDirectory) && tree.getDir(pathOrDirectory)

    if (directory) {
        return directory.subfiles.map(path => {
            const resolvedPath = `${pathOrDirectory}/${path}`
            const resource: YamlResource = {
                path: resolvedPath,
                data: parse(tree.readText(resolvedPath))
            }

            return resource
        })
    }

    if (file) {
        const resource: YamlResource = {
            path: file.path.toString(),
            data: parse(file.content.toString())
        }
    
        return [resource]
    }

    throw new SchematicsException(`not a file or directory: ${pathOrDirectory}`)
}

const updateKubernetesResource = (kind: string, pathOrDirectory: string, patches: (previous: Record<string, any>) => Record<string, any>): Rule => {
    return (tree: Tree): Tree => {
        const resource = getYamlResources(tree, pathOrDirectory).find(({ data }) => data.kind === kind)

        if (!resource) {
            throw new SchematicsException(`failed to find kubernetes resource of 'kind: ${kind}' in ${pathOrDirectory}`)
        }

        tree.overwrite(resource.path.toString(), stringify(patches(resource.data)))

        return tree
    }
}

const updateKustomization = (pathOrDirectory: string, patches: (previous: Record<string, any>) => Record<string, any>): Rule => {
    return (tree: Tree): Tree => {
        const resource = getYamlResources(tree, pathOrDirectory).find(({ path }) => path.endsWith('kustomization.yaml'))

        if (!resource) {
            throw new SchematicsException(`failed to find 'kustomization.yaml' in ${pathOrDirectory}`)
        }

        tree.overwrite(resource.path, stringify(patches(resource.data)))

        return tree
    }
}

export const command = (): Rule => async (tree) => {
    const projectConfigPath = `./argo-composer.config.yaml`
    const file = tree.read(projectConfigPath)?.toString()

    if (!file) {
        throw new SchematicsException('no initialized project! Please start from init command!')
    }

    const projects = tree.getDir('projects').subdirs
    const projectChoices = projects.map(projectPath => ({
        name: projectPath.toString(),
        value: projectPath.toString()
    }))

    const fromProjectName = await select({ message: 'Select project to pick app from', choices: projectChoices })
    const toProjectName = await select({
        message: 'Select project to move app to',
        choices: projectChoices.filter(({ name }) => name !== fromProjectName)
    })

    const applications = tree.getDir(`projects/${fromProjectName}/apps`).subdirs
    const applicationChoices = applications.map(applicationPath => ({
        name: applicationPath.toString(),
        value: applicationPath.toString()
    }))

    const applicationName = await select({ message: `Select ${fromProjectName} application`, choices: applicationChoices })

    if (tree.exists(`projects/${toProjectName}/apps/${applicationName}`)) {
        throw new SchematicsException(`application '${applicationName}' already exists in project '${toProjectName}'`)
    }

    const from = `projects/${fromProjectName}/apps/${applicationName}`
    const to = `projects/${toProjectName}/apps/${applicationName}`
    
    return chain([
        move(from, to),
        updateKustomization(
            `projects/${fromProjectName}/apps/kustomization.yaml`,
            (previous) => ({
                ...previous,
                resources: previous?.resources.filter((resource: string) => resource !== `./${applicationName}`) || []
            })
        ),
        updateKustomization(
            `projects/${toProjectName}/apps/kustomization.yaml`,
            (previous) => ({
                ...previous,
                resources: [
                    ...previous?.resources || [],
                    `./${applicationName}`
                ]
            })
        ),
        updateKubernetesResource(
            'Application',
            `projects/${toProjectName}/apps/${applicationName}`,
            (previous) => {
                const sourcePath = previous?.spec?.source?.path

                if (sourcePath) {
                    return {
                        ...previous,
                        spec: {
                            ...previous?.spec,
                            source: {
                                ...previous.spec.source,
                                path: sourcePath.replace(`./projects/${fromProjectName}`, `./projects/${toProjectName}`)
                            },
                            project: toProjectName
                        }
                    }
                }

                return {
                    ...previous,
                    spec: {
                        ...previous?.spec,
                        project: toProjectName
                    }
                }
            }
        ),
        // todo: cleaning up `from` directory - currently it is being left empty after move (very weird behavior!)
    ])
}
