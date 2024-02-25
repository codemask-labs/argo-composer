import { PodRestartPolicy } from 'resources/enums'
import { Containers } from '../common'

export type PodSpecification = {
    containers: Containers
    imagePullSecrets?: Array<string>
    restartPolicy?: PodRestartPolicy
}
