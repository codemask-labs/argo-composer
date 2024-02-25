import { Kind } from 'lib/enums'
import { Destination, Metadata, Source, SyncPolicy } from 'lib/resources'

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
