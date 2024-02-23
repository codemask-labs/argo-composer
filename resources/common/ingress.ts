import { concat, mergeDeepWith } from 'ramda'
import { Kind } from '../../enums'
import { DeepPartial } from '../../types'

export type Ingress = {
    apiVersion: string
    kind: Kind.Ingress
}

export const DEFAULT_INGRESS_RESOURCE: Ingress = {
    apiVersion: '',
    kind: Kind.Ingress
}

export const getIngress = (overrides?: DeepPartial<Ingress>) => mergeDeepWith(
    concat,
    DEFAULT_INGRESS_RESOURCE,
    overrides
)
