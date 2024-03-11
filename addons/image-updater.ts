import { AddonResource } from '../types'
import { Application } from '../types/resources'

const application: Application = {
    apiVersion: 'argoproj.io/v1alpha1',
    kind: 'Application',
    metadata: {
        name: 'image-updater',
        namespace: 'argocd'
    },
    spec: {
        project: 'default',
        revisionHistoryLimit: 0,
        destination: {
            server: 'https://kubernetes.default.svc',
            namespace: 'image-updater'
        },
        source: {
            chart: 'argocd-image-updater',
            repoURL: 'https://argoproj.github.io/argo-helm',
            targetRevision: '0.9.2'
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

export const IMAGE_UPDATER_ADDON_RESOURCE: AddonResource<Application> = {
    name: 'image-updater',
    resource: application
}
