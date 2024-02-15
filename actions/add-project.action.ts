import { input } from '@inquirer/prompts'
import { outputFile, exists, readFileSync } from 'fs-extra'
import { join } from 'node:path'
import { stringify, parse } from 'yaml'
import { getProjectConfig } from '../utils'
import { appProject } from '../resources'

export const addProjectAction = async () => {
  const { mainRepositoryUrl } = getProjectConfig()

  const projectName = await input({
    message: 'What name would you like to use for the project?'
  })
  const isProjectExists = await exists(
    join(process.cwd(), `projects/${projectName}/`)
  )

  if (isProjectExists) {
    throw new Error('Project with that name already exists!')
  }

  const currentProjectsKustomizationFile = readFileSync(
    join(process.cwd(), 'projects', 'kustomization.yaml')
  ).toString()
  const currentProjectsKustomization = parse(currentProjectsKustomizationFile)
  const appProjectResource = appProject(projectName, mainRepositoryUrl)
  const kustomizationResource = {
    resources: ['./apps', './project.yaml']
  }

  // todo: consider move file operations to utils
  await outputFile(
    join(process.cwd(), 'projects', projectName, 'project.yaml'),
    stringify(appProjectResource)
  )

  await outputFile(
    join(process.cwd(), 'projects', projectName, 'kustomization.yaml'),
    stringify(kustomizationResource)
  )

  await outputFile(
    join(process.cwd(), 'projects', 'kustomization.yaml'),
    stringify({
      resources: [...currentProjectsKustomization.resources, `./${projectName}`]
    })
  )

  await outputFile(
    join(process.cwd(), 'projects', 'apps', 'kustomization.yaml'),
    stringify({ resources: [] })
  )
}
