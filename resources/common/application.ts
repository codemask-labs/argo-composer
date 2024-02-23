import { mergeDeepWith, concat } from 'ramda'
import { Application, DeepPartial } from '../../types'
import { Kind } from '../../enums'

const DEFAULT_APPLICATION_RESOURCE: Application = {
    apiVersion: 'argoproj.io/v1alpha1',
    kind: Kind.Application,
    metadata: {
        name: 'default',
        namespace: 'argocd',
        annotations: {
            'argocd.argoproj.io/manifest-generate-paths': '.'
        },
        finalizers: [
            'resources-finalizer.argocd.argoproj.io'
        ]
    },
    spec: {
        project: 'default',
        revisionHistoryLimit: 0,
        source: {
            repoURL: 'https://github.com/example/argocd-resources',
            targetRevision: 'main',
            path: './projects/default/apps/default'
        },
        destination: {
            namespace: 'default-default',
            server: 'https://kubernetes.default.svc'
        },
        syncPolicy: {
            automated: {
                prune: true,
                selfHeal: true
            },
            syncOptions: [
                'ApplyOutOfSyncOnly=true',
                'PruneLast=true',
                'CreateNamespace=true'
            ],
            retry: {
                limit: 3,
                backoff: {
                    factor: 2,
                    duration: '5s',
                    maxDuration: '3m'
                }
            }
        }
    }
}

export const getApplication = (overrides?: DeepPartial<Application>): Application => mergeDeepWith(
    concat,
    DEFAULT_APPLICATION_RESOURCE,
    overrides
)
