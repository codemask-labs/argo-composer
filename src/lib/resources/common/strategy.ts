import { DeploymentStrategy } from 'lib/enums'
import { RollingUpdate } from './rolling-update'

export type Strategy = {
    type: DeploymentStrategy
    rollingUpdate: RollingUpdate
}
