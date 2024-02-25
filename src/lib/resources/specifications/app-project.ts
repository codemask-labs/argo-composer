import { Kind } from 'lib/enums'
import { Destination, Metadata, ResourceWhitelist } from 'lib/resources'

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
