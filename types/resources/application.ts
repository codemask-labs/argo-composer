export type Application = {
    apiVersion: string
    kind: string
    metadata: {
        name: string
        // You'll usually want to add your resources to the argocd namespace.
        namespace: string
        // Add this finalizer ONLY if you want these to cascade delete.
        finalizers?: Array<string>
        labels?: {
            [key: string]: string,
            name: string,
        }
    }
    spec: ApplicationSpec
}

type ApplicationSpec = {
    // The project the application belongs to.
    project: string
    destination?: Destination
    revisionHistoryLimit: number
    source: Source
    syncPolicy: SyncPolicy
    sources?: Array<{
        repoURL: string
        targetRevision: string
        path: string
        ref: string
    }>
    info?: Array<{
        name: string
        value: string
    }>
    ignoreDifferences?: Array<{
        group?: string
        kind?: string
        jsonPointers?: Array<string>
        jqPathExpressions?: Array<string>
        managedFieldsManagers?: Array<{
            name?: string
            namespace?: string
        }>
    }>
}

type HelmParameter = {
    name: string,
    value: string,
    forceString?: boolean
}

type FileParameter = {
    name: string,
    path: string
}

type HelmSource = {
    passCredentials?: boolean
    parameters?: Array<HelmParameter>
    fileParameters?: Array<FileParameter>
    releaseName?: string
    valueFiles?: Array<string>
    ignoreMissingValueFiles?: boolean
    values?: string
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    valuesObject?: Record<string, any>
    skipCrds?: boolean
    version?: string
}

type Replica = {
    name: string
    count: number
}

type Kustomize = {
    version: string
    namePrefix: string
    nameSuffix: string
    commonLabels: Record<string, string>
    commonAnnotations: Record<string, string>
    commonAnnotationsEnvsubst: boolean
    images: Array<string>
    namespace: string
    replicas: Array<Replica>
}

type JsonNet = {
    extVars: Array<{
        name: string
        value: string
        code?: boolean
    }>
    tlas: Array<{
        code: boolean
        name: string
        value: string
    }>
}

type Directory = {
    recurse: boolean
    jsonnet?: JsonNet
    exclude: string
    include: string
}

type PluginParameter = {
    name: string
    string?: string
    array?: Array<string>
    map?: Record<string, string>
}

type PluginEnv = {
    name: string
    value: string
}

type Plugin = {
    name: string
    env: Array<PluginEnv>
    parameters: Array<PluginParameter>
}

type Source = {
    repoURL: string
    targetRevision: string
    path?: string
    chart?: string
    helm?: HelmSource
    kustomize?: Kustomize
    directory?: Directory
    plugin?: Plugin
}

type Destination = {
    server?: string
    name?: string
    namespace: string
}

type Backoff = {
    duration: string
    factor: number
    maxDuration: string
}

type Retry = {
    limit: number
    backoff: Backoff
}

type Automated = {
    prune: boolean
    selfHeal: boolean
    allowEmpty?: boolean
}

type SyncPolicy = {
    automated: Automated
    syncOptions: Array<string>
    retry: Retry
}
