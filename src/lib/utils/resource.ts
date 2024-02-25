import { join } from 'node:path'
import { getAppProject, getKustomization } from 'resources/utils'
import { readYamlFile, writeYamlFile } from './file'
import { getProjectConfig } from './project'

type CreateProjectOptions = {
    root?: string
}

export type AppProjectActionHandler = (project: AppProjectActions) => Promise<void>

export type AppProjectActions = {
    addApplication: (application: Application) => Promise<void>
}

export const createAppProject = async (name: string, options?: CreateProjectOptions): Promise<AppProjectActions> => {
    const root = options?.root ? join(options.root, name) : join(process.cwd(), name)
    const appsKustomizationPath = join(root, 'apps', 'kustomization.yaml')
    const { repoURL } = await getProjectConfig()

    const appProject = getAppProject({
        metadata: { name },
        spec: {
            sourceRepos: [
                repoURL
            ]
        }
    })

    const appsKustomization = getKustomization({
        resources: []
    })

    const projectKustomization = getKustomization({
        resources: [
            'apps',
            'project.yaml'
        ]
    })

    await writeYamlFile(appsKustomizationPath, appsKustomization)
    await writeYamlFile(join(root, 'project.yaml'), appProject)
    await writeYamlFile(join(root, `kustomization.yaml`), projectKustomization)

    return {
        addApplication: async (application: Application) => {
            const current = await readYamlFile<Kustomization>(appsKustomizationPath)
            const next = getKustomization({
                resources: [...current.resources, application.metadata.name]
            })

            await writeYamlFile(appsKustomizationPath, next)
        }
    }
}
