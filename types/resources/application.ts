type Application = {
    apiVersion: string
    kind: string
    metadata: {
        name: string
        // You'll usually want to add your resources to the argocd namespace.
        namespace: string
        // Add this finalizer ONLY if you want these to cascade delete.
        finalizers: Array<string>
        labels: {
            name: string,
            [key: string]: string
        }
    }
    spec: ApplicationSpec
}

type ApplicationSpec = {
    // The project the application belongs to.
    project: string
    source: Source
    sources: Array<{
        repoURL: string
        targetRevision: string
        path: string
        ref: string
    }>
    info: Array<{
        name: string
        value: string
    }>
    destination: Destination
    syncPolicy: SyncPolicy
    ignoreDifferences: Array<{
        group?: string
        kind?: string
        jsonPointers?: Array<string>
        jqPathExpressions?: Array<string>
        managedFieldsManagers?: Array<{
            name?: string
            namespace?: string
        }>
    }>
    revisionHistoryLimit: number
}

type Source = {
    repoURL: string
    targetRevision: string
    path: string
    chart?: string
    helm?: {
        passCredentials: boolean
        parameters: Array<{ name: string, value: string, forceString?: boolean }>
        fileParameters: Array<{ name: string, path: string }>
        releaseName: string
        valueFiles: string[]
        ignoreMissingValueFiles: boolean
        values: string
        valuesObject: {
            ingress: {
                enabled: boolean
                path: string
                hosts: string[]
                annotations: Record<string, string>
                labels: Record<string, string>
                tls: Array<{
                    secretName: string
                    hosts: string[]
                }>
            }
        }
        skipCrds: boolean
        version: string
    }
    kustomize?: {
        version: string
        namePrefix: string
        nameSuffix: string
        commonLabels: Record<string, string>
        commonAnnotations: Record<string, string>
        commonAnnotationsEnvsubst: boolean
        images: Array<string>
        namespace: string
        replicas: Array<{
            name: string
            count: number
        }>
    }
    directory?: {
        recurse: boolean
        jsonnet?: {
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
        exclude: string
        include: string
    }
    plugin?: {
        name: string
        env: Array<{
            name: string
            value: string
        }>
        parameters: Array<{
            name: string
            string?: string
            array?: string[]
            map?: Record<string, string>
        }>
    }
}

type Destination = {
    server?: string
    name?: string
    namespace: string
}

type SyncPolicy = {
    automated: {
        prune: boolean
        selfHeal: boolean
        allowEmpty: boolean
    }
    syncOptions: Array<string>
    retry: {
        limit: number
        backoff: {
            duration: string
            factor: number
            maxDuration: string
        }
    }
}
