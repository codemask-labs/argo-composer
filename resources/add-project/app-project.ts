import { AppProject } from '../../types'

export const appProject = (name: string, mainRepositoryUrl: string): AppProject => ({
    apiVersion: 'argoproj.io/v1alpha1',
    kind: 'AppProject',
    metadata: {
        name,
        namespace: 'argocd', // maybe it should be in action
        finalizers: ['resources-finalizer.argocd.argoproj.io']
    },
    spec: {
        description: 'Project',
        sourceRepos: [mainRepositoryUrl],
        destinations: [{ namespace: '*', server: '*' }],
        clusterResourceWhitelist: [{ group: '*', kind: '*' }],
        namespaceResourceWhitelist: [{ group: '*', kind: '*' }]
    }
})
