import { input } from '@inquirer/prompts'
import { getProjectConfig, isPathExists, readYamlFile, writeYamlFile } from '../utils'
import { appProject } from '../resources'
import { Kustomization } from '../types'

export const addProjectAction = async () => {
    const { repoURL } = getProjectConfig()

    const projectName = await input({
        message: 'What name would you like to use for the project?'
    })
    const isProjectExists = isPathExists(`projects/${projectName}`)

    if (isProjectExists) {
        throw new Error('Project with that name already exists!')
    }

    const currentProjectsKustomization = await readYamlFile<Kustomization>('projects/kustomization.yaml')
    const appProjectResource = appProject(projectName, repoURL)
    const kustomizationResource: Kustomization = {
        resources: ['./apps', './project.yaml']
    }

    await writeYamlFile(`projects/${projectName}/project.yaml`, appProjectResource)
    await writeYamlFile(`projects/${projectName}/kustomization.yaml`, kustomizationResource)
    await writeYamlFile(`projects/${projectName}/apps/kustomization.yaml`, { resources: [] })
    await writeYamlFile(`projects/kustomization.yaml`, {
        resources: [...(currentProjectsKustomization?.resources ?? []), `./${projectName}`]
    })
}
