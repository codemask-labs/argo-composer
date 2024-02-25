import { Kind } from 'lib/enums'

export type HorizontalPodAutoscaler = {
    apiVersion: string
    kind: Kind.HPA
}
