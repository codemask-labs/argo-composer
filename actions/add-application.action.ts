import { join } from 'node:path'
import { confirm, input, select } from '@inquirer/prompts'
import { getDirectoryList, getProjectConfig, isDirectory, writeYamlFile } from '../utils'
import { createApplication } from '../resources/utils'

const getApplicationDestination = async () => {
    const projects = getDirectoryList('projects')

    const applicationName = await input({
        message: 'What is the name of application?'
    })

    const projectName = await select({
        message: `To which project we are adding '${applicationName}'?`,
        choices: projects.map(value => ({ value }))
    })

    const applicationDirectory = join(process.cwd(), 'projects', projectName, 'apps', applicationName)

    if (isDirectory(applicationDirectory)) {
        throw new Error(`Application '${applicationName}' already exists in '${projectName}'`)
    }

    return {
        applicationName,
        projectName
    }
}

export const getApplicationNamespace = (environment: string, applicationName: string, projectName: string) =>
    projectName === 'default'
        ? `${applicationName}-${environment}`
        : `${projectName}-${applicationName}-${environment}`

export const addApplicationAction = async () => {
    const config = getProjectConfig()
    const { applicationName, projectName } = await getApplicationDestination()

    const imageName = await input({
        message: 'What is the image name (for example: your-registry.com/your-app, nginx:latest etc.)?'
    })

    const containerPort = await input({
        message: 'What is the image container port?'
    })

    const servicePort = await input({
        message: 'What is the cluster service port?'
    })

    const useOverlays = await confirm({
        message: 'Use overlays (multiple environments)?',
        default: true
    })

    const useImageUpdater = await confirm({
        message: 'Use image-updater?',
        default: true
    })

    const useHorizontalPodAutoscaler = await confirm({
        message: 'Use HPA (Horizontal Pod Autoscaler)?',
        default: true
    })

    const applicationDirectory = `projects/${projectName}/apps/${applicationName}`
    const applications = config.environments.map(
        environment => createApplication({
            name: `${applicationName}-${environment}`,
            namespace: getApplicationNamespace(environment, applicationName, projectName),
            project: projectName,
            repoURL: config.repoURL
        })
    )

    if (useOverlays) {
        await writeYamlFile(`${applicationDirectory}/base/configmap.yaml`, {})
        await writeYamlFile(`${applicationDirectory}/base/deployment.yaml`, {})
        await writeYamlFile(`${applicationDirectory}/base/hpa.yaml`, {})
        await writeYamlFile(`${applicationDirectory}/base/service.yaml`, {})
        await writeYamlFile(`${applicationDirectory}/base/ingress.yaml`, {})

        // eslint-disable-next-line no-loops/no-loops
        for (const environment of config.environments) {
            await writeYamlFile(`${applicationDirectory}/overlays/${environment}/patches.yaml`, {})
            await writeYamlFile(`${applicationDirectory}/overlays/${environment}/kustomization.yaml`, {
                resources: ['../../base'],
                // images: [{ name: imageUrlOrName }],
                patches: [{ path: './patches.yaml' }]
            })
        }

        await writeYamlFile(`${applicationDirectory}/base/kustomization.yaml`, {
            resources: [
                './configmap.yaml',
                './deployment.yaml',
                './hpa.yaml',
                './service.yaml',
                './ingress.yaml'
            ]
        })
    }

    await writeYamlFile(`${applicationDirectory}/application.yaml`, applications)
    await writeYamlFile(`${applicationDirectory}/kustomization.yaml`, {
        resources: ['./application.yaml']
    })

    // const appName = await input({
    //     message: 'What name would you like to use for the app?'
    // })
    // const imageURL = await input({
    //     message: 'Image URL, example: your-registry.com/your-app'
    // })
    // const projectName = await select({
    //     message: 'Please select project where you want add your app',
    //     choices: currentProjects.map(currentProject => ({
    //         name: currentProject.toString(),
    //         value: currentProject.toString()
    //     }))
    // })
    // const appPort = await input({ message: 'Provide port of the app' })
    // const useImageAutoUpdater = await confirm({
    //     message: 'Use image auto-updater?',
    //     default: true
    // })
    // const useHorizontalPodAutoscaler = await confirm({
    //     message: 'Use HPA (Horizontal Pod Autoscaler)?',
    //     default: true
    // })

    // const options: Options = {
    //     appName,
    //     imageURL,
    //     projectName,
    //     appPort,
    //     useImageAutoUpdater,
    //     environments,
    //     shouldAppContainOverlays,
    //     useHorizontalPodAutoscaler
    // }

    // const base = shouldAppContainOverlays ? [addOverlayBase(options)] : []
    // const overlays = shouldAppContainOverlays ? environments.map(env => addOverlay(env, options)) : []
    // const resources = !shouldAppContainOverlays ? [addResources(options)] : []
    // const templateSource = apply(url('./files'), [
    //     template({ ...options, ...strings, mainProjectName, mainRepoURL }),
    //     move(`/projects/${options.projectName}/apps/`)
    // ])

}
