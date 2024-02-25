import { Kind } from 'resources/enums'

export type HorizontalPodAutoscaler = {
    apiVersion: string
    kind: Kind.HPA
}
