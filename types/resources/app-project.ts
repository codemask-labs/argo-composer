import { Kind } from '../../enums'
import { Metadata } from './metadata'

export type AppProject = {
    apiVersion: string
    kind: Kind.AppProject
    metadata: Metadata
    spec: {
        description: string
        sourceRepos: Array<string>
        destinations: Array<Destination>
        clusterResourceWhitelist: Array<ResourceWhitelist>
        namespaceResourceWhitelist: Array<ResourceWhitelist>
    }
}

type Destination = {
    namespace: string
    server: string
}

type ResourceWhitelist = {
    group: string
    kind: string
}
