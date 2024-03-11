type Patch = {
    path: string
}

export type Kustomization = {
    resources: Array<string>
    base?: string
    pathes?: Array<Patch>
}
