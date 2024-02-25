import { Kind } from 'lib/enums'
import { Metadata, Port, Selector } from '../common'

export type ServiceSpecification = {
    selector: Selector
    ports: Array<Port>
}

export type Service = {
    apiVersion: string
    kind: Kind.Service
    metadata: Metadata
    spec: ServiceSpecification
}
