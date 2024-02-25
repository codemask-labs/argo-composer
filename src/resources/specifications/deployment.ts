import { Kind } from 'resources/enums'
import { Metadata, Selector, Strategy } from '../common'
import { PodSpecification } from './pod'

type Template = {
    metadata: Metadata
    spec: PodSpecification
}

export type DeploymentSpecification = {
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
    spec: DeploymentSpecification
}
