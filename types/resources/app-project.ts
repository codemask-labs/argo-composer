export type AppProject = {
    apiVersion: string
    kind: string // maybe it should be an enum
    metadata: Metadata
    spec: {
        description: string
        sourceRepos: Array<string>
        destinations: Array<Destination>
        clusterResourceWhitelist: Array<ResourceWhitelist>
        namespaceResourceWhitelist: Array<ResourceWhitelist>
    }
}

type Metadata = {
    name: string
    namespace: string
    finalizers?: Array<string>
}

type Destination = {
    namespace: string
    server: string
}

type ResourceWhitelist = {
    group: string
    kind: string
}
