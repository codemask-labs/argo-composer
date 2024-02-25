import { DeploymentStrategy, Kind, PodRestartPolicy } from 'lib/enums'
import { AppProject, Application, ConfigMap, Deployment, HorizontalPodAutoscaler, Ingress, Kustomization, Service } from './specifications'

export const DEFAULT_APP_PROJECT_RESOURCE: AppProject = {
    apiVersion: 'argoproj.io/v1alpha1',
    kind: Kind.AppProject,
    metadata: {
        name: 'default',
        namespace: 'argocd',
        finalizers: ['resources-finalizer.argocd.argoproj.io']
    },
    spec: {
        description: 'Default AppProject description',
        sourceRepos: [],
        destinations: [{ namespace: '*', server: '*' }],
        clusterResourceWhitelist: [{ group: '*', kind: '*' }],
        namespaceResourceWhitelist: [{ group: '*', kind: '*' }]
    }
}

export const DEFAULT_APPLICATION_RESOURCE: Application = {
    apiVersion: 'argoproj.io/v1alpha1',
    kind: Kind.Application,
    metadata: {
        name: 'default',
        namespace: 'argocd',
        // annotations: {
        //     'argocd.argoproj.io/manifest-generate-paths': '.'
        // },
        finalizers: ['resources-finalizer.argocd.argoproj.io']
    },
    spec: {
        project: 'default',
        revisionHistoryLimit: 0,
        source: {
            repoURL: 'https://github.com/example/argocd-resources',
            targetRevision: 'main',
            path: '<change to path of application kustomization>'
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
            syncOptions: ['ApplyOutOfSyncOnly=true', 'PruneLast=true', 'CreateNamespace=true'],
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

export const DEFAULT_DEPLOYMENT_RESOURCE: Deployment = {
    apiVersion: 'apps/v1',
    kind: Kind.Deployment,
    metadata: {
        name: 'default',
        namespace: 'default'
    },
    spec: {
        progressDeadlineSeconds: 600,
        replicas: 1,
        revisionHistoryLimit: 1,
        selector: {
            matchLabels: {
                app: 'default'
            }
        },
        strategy: {
            type: DeploymentStrategy.RollingUpdate,
            rollingUpdate: {
                maxSurge: '100%',
                maxUnavailable: 0
            }
        },
        template: {
            metadata: {
                name: 'default',
                namespace: 'default'
            },
            spec: {
                restartPolicy: PodRestartPolicy.Always,
                containers: []
            }
        }
    }
}

export const DEFAULT_HPA_RESOURCE: HorizontalPodAutoscaler = {
    apiVersion: 'v1',
    kind: Kind.HPA
}

export const DEFAULT_SERVICE_RESOURCE: Service = {
    apiVersion: 'v1',
    kind: Kind.Service,
    metadata: {
        name: '',
        namespace: '',
        annotations: {
            'argocd.argoproj.io/manifest-generate-paths': '.'
        },
        finalizers: ['resources-finalizer.argocd.argoproj.io']
    },
    spec: {
        selector: {
            matchLabels: {
                app: 'default'
            }
        },
        ports: []
    }
}

export const DEFAULT_INGRESS_RESOURCE: Ingress = {
    apiVersion: 'v1',
    kind: Kind.Ingress
}

export const DEFAULT_CONFIG_MAP_RESOURCE: ConfigMap = {
    apiVersion: 'v1',
    kind: Kind.ConfigMap
}

export const DEFAULT_KUSTOMIZATION_RESOURCE: Kustomization = {
    resources: []
}
