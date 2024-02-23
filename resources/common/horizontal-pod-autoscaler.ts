import { concat, mergeDeepWith } from 'ramda'
import { Kind } from '../../enums'
import { DeepPartial } from '../../types'

export type HorizontalPodAutoscaler = {
    apiVersion: string
    kind: Kind.HPA
}

export const DEFAULT_HPA_RESOURCE: HorizontalPodAutoscaler = {
    apiVersion: 'v1',
    kind: Kind.HPA
}

export const getHorizontalPodAutoscaler = (overrides?: DeepPartial<HorizontalPodAutoscaler>): HorizontalPodAutoscaler => mergeDeepWith(
    concat,
    DEFAULT_HPA_RESOURCE,
    overrides
)
