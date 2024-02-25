import { Kind } from 'resources/enums'
import { Destination, Metadata, ResourceWhitelist } from 'resources/utils'

export type AppProjectSpecification = {
    description: string
    sourceRepos: Array<string>
    destinations: Array<Destination>
    clusterResourceWhitelist: Array<ResourceWhitelist>
    namespaceResourceWhitelist: Array<ResourceWhitelist>
}

export type AppProject = {
    apiVersion: string
    kind: Kind.AppProject
    metadata: Metadata
    spec: AppProjectSpecification
}
