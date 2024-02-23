import { concat, mergeDeepWith } from 'ramda'
import { Kind } from '../../enums'
import { DeepPartial } from '../../types'
import { Metadata } from '../../types/resources/metadata'

type MatchLabels = {
    app: string
}

type Selector = {
    matchLabels: MatchLabels
}

enum DeploymentStrategy {
    RollingUpdate = 'RollingUpdate'
}

type RollingUpdate = {
    maxUnavailable: number
    maxSurge: string
}

type Strategy = {
    type: DeploymentStrategy
    rollingUpdate: RollingUpdate
}

enum ImagePullPolicy {
    IfNotPresent = 'IfNotPresent'
}

type Port = {
    containerPort: number
    name?: string
}

type Probe = {
    httpGet: {
        path: 'string'
        port: number
    }
    initialDelaySeconds: number
    timeoutSeconds: 3
    successThreshold: number
    failureThreshold: number
    periodSeconds: number
}

type SecurityContext = {
    allowPrivilegeEscalation: boolean
    runAsNonRoot?: boolean
    runAsUser?: number
}

type Container = {
    name: string
    image: string
    imagePullPolicy?: ImagePullPolicy
    ports?: Array<Port>
    livenessProbe?: Probe
    redinessProbe?: Probe
    securityContext?: SecurityContext
}

type Containers = Array<Container>

enum PodRestartPolicy {
    Always = 'Always'
}

type PodSpecification = {
    containers: Containers
    imagePullSecrets?: Array<string>
    restartPolicy?: PodRestartPolicy
}

type Template = {
    metadata: Metadata
    spec: PodSpecification
}

type Specification = {
    selector: Selector
    replicas: number
    strategy: Strategy
    template: Template
    revisionHistoryLimit: number
    progressDeadlineSeconds: number
}

export type Deployment = {
    apiVersion: string
    kind: Kind.Deployment
    metadata: Metadata
    spec: Specification
}

export const DEFAULT_CONTAINER_RESOURCE: Container = {
    name: 'example-container-name',
    image: 'your-registry.com/your-app',
    imagePullPolicy: ImagePullPolicy.IfNotPresent,
    ports: [
        { name: 'http-3000', containerPort: 3000 }
    ],
    securityContext: {
        allowPrivilegeEscalation: false,
        runAsNonRoot: true
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
                containers: [
                    DEFAULT_CONTAINER_RESOURCE
                ]
            }
        }
    }
}

export const getDeployment = (overrides?: DeepPartial<Deployment>): Deployment => mergeDeepWith(
    concat,
    DEFAULT_DEPLOYMENT_RESOURCE,
    overrides
)
