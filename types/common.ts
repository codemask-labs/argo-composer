export type ProjectConfig = {
    mainProjectName: string
    mainRepositoryUrl: string
    environments: Array<string>
}

export type AddonResource<T> = {
    name: string
    resource: T
}

export type Metadata = {
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

export type DeepPartial<T> = {
    [P in keyof T]?: T[P] extends object ? DeepPartial<T[P]> : T[P];
}
