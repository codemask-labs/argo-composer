import { PodRestartPolicy } from 'lib/enums'
import { Containers } from '../common'

export type PodSpecification = {
    containers: Containers
    imagePullSecrets?: Array<string>
    restartPolicy?: PodRestartPolicy
}
