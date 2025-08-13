export type ConfigInput = {
    inputs?: Array<{ name: string }>
}

export type ScaffoldOptions = {
    rootDirectory: string
    overwrite?: boolean
    mainRepositoryUrl: string
    environments: Array<string>
}

export type TemplateFile = {
    path: string
    content: string
}
