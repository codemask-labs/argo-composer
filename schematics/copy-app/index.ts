import { parseAllDocuments, stringify } from 'yaml'
import { join } from 'node:path'
import { Rule, SchematicsException, Tree, chain } from '@angular-devkit/schematics'
import { select } from '@inquirer/prompts'

type YamlResource = {
    path: string
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    documents: Array<Record<string, any> | null>
}

const getYamlResources = (tree: Tree, pathOrDirectory: string): Array<YamlResource> => {
    const exists = tree.exists(pathOrDirectory)
    const file = exists && tree.get(pathOrDirectory)
    const directory = !exists && tree.getDir(pathOrDirectory)

    if (directory) {
        return directory.subfiles.map(path => {
            const resolvedPath = `${pathOrDirectory}/${path}`
            const resource: YamlResource = {
                path: resolvedPath,
                documents: parseAllDocuments(tree.readText(resolvedPath)).map(document => document.toJS())
            }

            return resource
        })
    }

    if (file) {
        const resource: YamlResource = {
            path: file.path.toString(),
            documents: parseAllDocuments(file.content.toString()).map(document => document.toJS())
        }

        return [resource]
    }

    throw new SchematicsException(`Not a file or directory: ${pathOrDirectory}`)
}

const updateKubernetesResourceByKind =
    (
        kind: string,
        pathOrDirectory: string,
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        patches: (previous: Record<string, any>) => Record<string, any>
    ): Rule =>
    (tree: Tree): Tree => {
        const resource = getYamlResources(tree, pathOrDirectory).find(({ documents }) => documents.some(document => document?.kind === kind))

        if (!resource) {
            throw new SchematicsException(`Failed to find kubernetes resource of 'kind: ${kind}' in ${pathOrDirectory}`)
        }

        tree.overwrite(
            resource.path.toString(),
            stringify(
                resource.documents.map(document => {
                    if (document?.kind === kind) {
                        return patches(document)
                    }

                    return document
                })
            )
        )

        return tree
    }

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const updateKustomization =
    (pathOrDirectory: string, patches: (previous: Record<string, any>) => Record<string, any>): Rule =>
    (tree: Tree): Tree => {
        const resource = getYamlResources(tree, pathOrDirectory).find(({ path }) => path.endsWith('kustomization.yaml'))

        if (!resource) {
            throw new SchematicsException(`Failed to find 'kustomization.yaml' in ${pathOrDirectory}`)
        }

        tree.overwrite(resource.path, stringify(patches(resource.documents)))

        return tree
    }

const updateKubernetesApplication = (applicationName: string, sourceProjectName: string, destProjectName: string) =>
    updateKubernetesResourceByKind('Application', `projects/${destProjectName}/apps/${applicationName}`, previous => {
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
    })

const copyDir =
    (source: string, dest: string): Rule =>
    tree => {
        tree.getDir(source).visit(path => {
            const relativePath = path.slice(source.length + 1)
            const content = tree.read(path)

            if (content) {
                tree.create(join(dest, relativePath), content)
            }
        })

        return tree
    }

export const command = (): Rule => async tree => {
    const projectConfigPath = `./argo-composer.config.yaml`
    const file = tree.read(projectConfigPath)?.toString()

    if (!file) {
        throw new SchematicsException('No initialized project found! Please start from init command!')
    }

    const projects = tree.getDir('projects').subdirs
    const projectChoices = projects.map(projectPath => ({
        name: projectPath.toString(),
        value: projectPath.toString()
    }))

    const sourceProjectName = await select({
        message: 'Select source project',
        choices: projectChoices
    })

    const applications = tree.getDir(`projects/${sourceProjectName}/apps`).subdirs
    const applicationChoices = applications.map(applicationPath => ({
        name: applicationPath.toString(),
        value: applicationPath.toString()
    }))

    const applicationName = await select({
        message: `Select source application from ${sourceProjectName} project`,
        choices: applicationChoices
    })

    const destProjectName = await select({
        message: 'Select destination project',
        choices: projectChoices.filter(({ name }) => name !== sourceProjectName)
    })

    if (tree.exists(`projects/${destProjectName}/apps/${applicationName}`)) {
        throw new SchematicsException(`Application '${applicationName}' already exists in project '${destProjectName}'`)
    }

    const source = `projects/${sourceProjectName}/apps/${applicationName}`
    const dest = `projects/${destProjectName}/apps/${applicationName}`

    return chain([
        copyDir(source, dest),
        updateKustomization(`projects/${destProjectName}/apps/kustomization.yaml`, previous => ({
            ...previous,
            resources: [...(previous?.resources || []), `./${applicationName}`]
        })),
        updateKubernetesApplication(applicationName, sourceProjectName, destProjectName)
    ])
}
