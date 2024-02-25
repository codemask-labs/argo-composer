import { ImagePullPolicy } from 'resources/enums'
import { Port } from './port'
import { Probe } from './probe'
import { SecurityContext } from './security-context'

export type Container = {
    name: string
    image: string
    imagePullPolicy?: ImagePullPolicy
    ports?: Array<Port>
    livenessProbe?: Probe
    redinessProbe?: Probe
    securityContext?: SecurityContext
}

export type Containers = Array<Container>
