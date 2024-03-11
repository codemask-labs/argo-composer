import { join } from 'node:path'
import { confirm, input, select } from '@inquirer/prompts'
import { Application, ProjectConfig } from '../types'
import { getDirectoryList, getProjectConfig, isDirectory, writeYamlFile } from '../utils'
import { createApplication } from '../resources'

type ApplicationOptions = {
    config: ProjectConfig,
    imageName: string
    projectName: string
    applicationName: string
    applicationDirectory: string
    containerPort: number
    servicePort: number
    useHorizontalPodAutoscaler: boolean
    useImageUpdater: boolean
}

const getApplicationDestination = async () => {
    const projects = getDirectoryList('projects')

    const applicationName = await input({
        message: 'What is the name of application?'
    })

    const projectName = await select({
        message: `To which project we are adding '${applicationName}' application?`,
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

export const addApplicationsWithOverlay = async (options: ApplicationOptions): Promise<Array<Application>> => {
    const { projectName, applicationName, applicationDirectory, config } = options

    const applications = config.environments.map(
        environment => createApplication({
            name: `${applicationName}-${environment}`,
            namespace: getApplicationNamespace(environment, applicationName, projectName),
            project: projectName,
            repoURL: config.repoURL,
            path: `${applicationDirectory}/overlays/${environment}`
        })
    )

    if (options.useHorizontalPodAutoscaler) {
        await writeYamlFile(`${applicationDirectory}/base/hpa.yaml`, {})
    }

    await writeYamlFile(`${applicationDirectory}/base/configmap.yaml`, {})
    await writeYamlFile(`${applicationDirectory}/base/deployment.yaml`, {})
    await writeYamlFile(`${applicationDirectory}/base/service.yaml`, {})
    await writeYamlFile(`${applicationDirectory}/base/ingress.yaml`, {})
    await writeYamlFile(`${applicationDirectory}/base/kustomization.yaml`, {
        resources: [
            './configmap.yaml',
            './deployment.yaml',
            './service.yaml',
            './ingress.yaml',
            ...options.useHorizontalPodAutoscaler ? ['./hpa.yaml'] : []
        ]
    })

    await Promise.all(config.environments.map(async environment => {
        await writeYamlFile(`${applicationDirectory}/overlays/${environment}/patches.yaml`, {})
        await writeYamlFile(`${applicationDirectory}/overlays/${environment}/kustomization.yaml`, {
            resources: ['../../base'],
            // images: [{ name: imageUrlOrName }],
            patches: [{ path: './patches.yaml' }]
        })
    }))

    return applications
}

export const addApplicationWithResources = async (options: ApplicationOptions): Promise<Application> => {
    const { projectName, applicationName, applicationDirectory, config } = options

    const applicationPath = `${applicationDirectory}/resources`
    const application = createApplication({
        name: applicationName,
        namespace: applicationName,
        project: projectName,
        repoURL: config.repoURL,
        path: applicationPath
    })

    if (options.useHorizontalPodAutoscaler) {
        await writeYamlFile(`${applicationPath}/hpa.yaml`, {})
    }

    await writeYamlFile(`${applicationPath}/configmap.yaml`, {})
    await writeYamlFile(`${applicationPath}/deployment.yaml`, {})
    await writeYamlFile(`${applicationPath}/service.yaml`, {})
    await writeYamlFile(`${applicationPath}/ingress.yaml`, {})
    await writeYamlFile(`${applicationPath}/kustomization.yaml`, {
        resources: [
            './configmap.yaml',
            './deployment.yaml',
            './service.yaml',
            './ingress.yaml',
            ...options.useHorizontalPodAutoscaler ? ['./hpa.yaml'] : []
        ]
    })

    return application
}

export const addApplicationAction = async () => {
    const config = getProjectConfig()
    const { applicationName, projectName } = await getApplicationDestination()

    const imageName = await input({
        message: 'What is the image name (for example: your-registry.com/your-app, nginx:latest etc.)?'
    })

    const containerPort = await input({
        message: 'What is the container port?'
    })

    const servicePort = await input({
        message: 'What is the service port?',
        default: containerPort
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
    const options: ApplicationOptions = {
        config,
        imageName,
        projectName,
        applicationName,
        applicationDirectory,
        containerPort: parseInt(containerPort, 10),
        servicePort: parseInt(servicePort, 10),
        useHorizontalPodAutoscaler,
        useImageUpdater
    }

    const applications = useOverlays
        ? await addApplicationsWithOverlay(options)
        : await addApplicationWithResources(options)

    await writeYamlFile(`${applicationDirectory}/application.yaml`, applications)
    await writeYamlFile(`${applicationDirectory}/kustomization.yaml`, {
        resources: ['./application.yaml']
    })
}
