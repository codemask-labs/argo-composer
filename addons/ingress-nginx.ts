import { AddonResource } from '../types'
import { Application } from '../resources'

const application: Application = {
    apiVersion: 'argoproj.io/v1alpha1',
    kind: 'Application',
    metadata: {
        name: 'ingress-nginx',
        namespace: 'argocd',
        annotations: {
            'argocd.argoproj.io/sync-wave': '-10'
        }
    },
    spec: {
        project: 'default',
        revisionHistoryLimit: 0,
        destination: {
            server: 'https://kubernetes.default.svc',
            namespace: 'ingress-nginx'
        },
        source: {
            chart: 'ingress-nginx',
            repoURL: 'https://kubernetes.github.io/ingress-nginx',
            targetRevision: '4.5.2'
        },
        syncPolicy: {
            automated: {
                prune: true,
                selfHeal: true
            },
            syncOptions: ['ApplyOutOfSyncOnly=true', 'PruneLast=true', 'CreateNamespace=true'],
            retry: {
                limit: 3,
                backoff: {
                    duration: '5s',
                    factor: 2,
                    maxDuration: '3m'
                }
            }
        }
    }
}

export const INGRESS_NGINX_ADDON_RESOURCE: AddonResource<Application> = {
    name: 'ingress-nginx',
    resource: application
}
