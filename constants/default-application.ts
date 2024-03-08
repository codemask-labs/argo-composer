import { Application } from '../types'

export const DEFAULT_APPLICATION: Application = {
    apiVersion: 'argoproj.io/v1alpha1',
    kind: 'Application',
    metadata: {
        name: 'default',
        namespace: 'argocd',
        finalizers: ['resources-finalizer.argocd.argoproj.io']
    },
    spec: {
        // https://argo-cd.readthedocs.io/en/stable/user-guide/projects/#the-default-project
        project: 'default',
        revisionHistoryLimit: 0,
        source: {
            repoURL: '<required>',
            targetRevision: 'main'
        },
        destination: {
            server: 'https://kubernetes.default.svc',
            namespace: 'default'
        },
        syncPolicy: {
            retry: {
                limit: 3,
                backoff: {
                    duration: '5s',
                    factor: 2,
                    maxDuration: '3m'
                }
            },
            automated: {
                prune: true,
                selfHeal: true
            },
            syncOptions: [
                'ApplyOutOfSyncOnly=true',
                'PruneLast=true'
            ]
        }
    }
}
