import { AddonResource } from '../types'
import { Application } from '../types/resources'

const application: Application = {
    apiVersion: 'argoproj.io/v1alpha1',
    kind: 'Application',
    metadata: {
        name: 'reflector',
        namespace: 'argocd'
    },
    spec: {
        project: 'default',
        revisionHistoryLimit: 0,
        destination: {
            server: 'https://kubernetes.default.svc',
            namespace: 'reflector'
        },
        source: {
            chart: 'reflector',
            repoURL: 'https://emberstack.github.io/helm-charts',
            targetRevision: '7.1.238'
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

export const REFLECTOR_ADDON_RESOURCE: AddonResource<Application> = {
    name: 'reflector',
    resource: application
}
