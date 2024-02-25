export type ArgoComposerConfig = {
    repo: string
    namespaces: Array<string>
}

export const getArgoComposerConfig = (path?: string): ArgoComposerConfig => ({
    repo: '',
    namespaces: []
})
