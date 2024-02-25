import { DeploymentStrategy } from 'resources/enums'
import { RollingUpdate } from './rolling-update'

export type Strategy = {
    type: DeploymentStrategy
    rollingUpdate: RollingUpdate
}
