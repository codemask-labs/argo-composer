export type ProjectConfig = {
    mainRepositoryUrl: string
    environments: Array<string>
}

export type AddonResource<T> = {
    name: string
    resource: T
}

export type DeepPartial<T> = {
    [P in keyof T]?: T[P] extends object ? DeepPartial<T[P]> : T[P]
}
