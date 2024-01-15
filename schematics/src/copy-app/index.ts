import { parse, stringify } from 'yaml'
import { join } from 'node:path'
import {
    Rule,
    SchematicsException,
    Tree,
    chain
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

const updateKubernetesApplication = (applicationName: string, sourceProjectName: string, destProjectName: string) =>
    updateKubernetesResource(
        'Application',
        `projects/${destProjectName}/apps/${applicationName}`,
        (previous) => {
            const sourcePath = previous?.spec?.source?.path

            if (sourcePath) {
                return {
                    ...previous,
                    spec: {
                        ...previous?.spec,
                        source: {
                            ...previous.spec.source,
                            path: sourcePath.replace(`./projects/${sourceProjectName}`, `./projects/${destProjectName}`)
                        },
                        project: destProjectName
                    }
                }
            }

            return {
                ...previous,
                spec: {
                    ...previous?.spec,
                    project: destProjectName
                }
            }
        }
    )

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

    const sourceProjectName = await select({
        message: 'Select source project', choices: projectChoices
    })

    const applications = tree.getDir(`projects/${sourceProjectName}/apps`).subdirs
    const applicationChoices = applications.map(applicationPath => ({
        name: applicationPath.toString(),
        value: applicationPath.toString()
    }))

    const applicationName = await select({
        message: `Select ${sourceProjectName} application`,
        choices: applicationChoices
    })

    const destProjectName = await select({
        message: 'Select project to move app to',
        choices: projectChoices.filter(({ name }) => name !== sourceProjectName)
    })

    if (tree.exists(`projects/${destProjectName}/apps/${applicationName}`)) {
        throw new SchematicsException(`application '${applicationName}' already exists in project '${destProjectName}'`)
    }

    const source = `projects/${sourceProjectName}/apps/${applicationName}`
    const dest = `projects/${destProjectName}/apps/${applicationName}`

    console.log(source)
    console.log(dest)

    return chain([
        (tree) => {
            tree.getDir(source).visit((path) => {
                const relativePath = path.slice(source.length + 1)
                const content = tree.read(path)

                console.log('content:', content)

                console.log('dest path:', join(dest, relativePath))

                if (content) {
                    tree.create(join(dest, relativePath), content)
                }
            })

            return tree
        },
        updateKustomization(
            `projects/${destProjectName}/apps/kustomization.yaml`,
            (previous) => ({
                ...previous,
                resources: [
                    ...previous?.resources || [],
                    `./${applicationName}`
                ]
            })
        ),
        updateKubernetesApplication(applicationName, sourceProjectName, destProjectName),
    ])
}
