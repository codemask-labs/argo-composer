export type DeepPartial<T> = T extends object ? { [P in keyof T]?: DeepPartial<T[P]> } : T

export type ProjectConfig = {
    mainProjectName: string
    mainRepositoryUrl: string
    environments: Array<string>
}
