import { ProjectConfig } from './common'

export type ApplicationOptions = {
    config: ProjectConfig
    imageName: string
    projectName: string
    applicationName: string
    applicationDirectory: string
    containerPort: number
    servicePort: number
    useHorizontalPodAutoscaler: boolean
    useImageUpdater: boolean
    useHealthCheck: boolean
    useSecurityContext: boolean
}
