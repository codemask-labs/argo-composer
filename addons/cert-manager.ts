import { AddonResource } from '../types'
import { Application } from '../resources'

const application: Application = {
    apiVersion: 'argoproj.io/v1alpha1',
    kind: 'Application',
    metadata: {
        name: 'cert-manager',
        namespace: 'argocd'
    },
    spec: {
        project: 'default',
        revisionHistoryLimit: 0,
        destination: {
            server: 'https://kubernetes.default.svc',
            namespace: 'cert-manager'
        },
        source: {
            chart: 'cert-manager',
            repoURL: 'https://charts.jetstack.io',
            targetRevision: '1.13.3',
            helm: {
                valuesObject: {
                    installCRDs: true
                }
            }
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

export const CERT_MANAGER_ADDON_RESOURCE: AddonResource<Application> = {
    name: 'cert-manager',
    resource: application
}
