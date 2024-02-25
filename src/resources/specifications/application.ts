import { Kind } from 'resources/enums'
import { Destination, Metadata, Source, SyncPolicy } from 'resources/utils'

export type ApplicationSpecification = {
    project: string
    source: Source
    destination: Destination
    syncPolicy: SyncPolicy
    revisionHistoryLimit?: number
}

export type Application = {
    apiVersion: string
    kind: Kind.Application
    metadata: Metadata
    spec: ApplicationSpecification
}
