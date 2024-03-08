import { AppProject } from '../types'

export const DEFAULT_APP_PROJECT: AppProject = {
    apiVersion: 'argoproj.io/v1alpha1',
    kind: 'AppProject',
    metadata: {
        name: 'default',
        namespace: 'argocd',
        finalizers: ['resources-finalizer.argocd.argoproj.io']
    },
    spec: {
        description: '<default project description>',
        sourceRepos: ['*'],
        destinations: [
            {
                namespace: '*',
                server: '*'
            }
        ],
        clusterResourceWhitelist: [
            {
                group: '*',
                kind: '*'
            }
        ],
        namespaceResourceWhitelist: [
            {
                group: '*',
                kind: '*'
            }
        ]
    }
}
