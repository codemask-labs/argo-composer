import { Kind } from '../../enums'
import { Metadata } from './metadata'

type Source = {
    repoURL: string
    targetRevision: string
    path: string
}

type Destination = {
    server: string
    namespace: string
}

type Automated = {
    prune: boolean
    selfHeal: boolean
}

type Backoff = {
    factor: number
    duration: string
    maxDuration: string
}

type Retry = {
    limit: number
    backoff: Backoff
}

type SyncPolicy = {
    automated: Automated
    syncOptions: Array<string>
    retry: Retry
}

type Specification = {
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
    spec: Specification
}
