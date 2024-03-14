import { ProjectConfig } from './common'

export type ApplicationOptions = {
    config: ProjectConfig
    imageName: string
    projectName: string
    applicationName: string
    applicationDirectory: string
    containerPort: number
    servicePort: number
    useIngress: boolean
    useHorizontalPodAutoscaler: boolean
    useImageUpdater: boolean
    useHealthCheck: boolean
    useSecurityContext: boolean
}
