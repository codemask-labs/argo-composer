import { concat, mergeDeepWith } from 'ramda'
import { Kind } from '../../enums'
import { DeepPartial } from '../../types'

export type Service = {
    apiVersion: string
    kind: Kind.Service
}

export const DEFAULT_SERVICE_RESOURCE: Service = {
    apiVersion: '',
    kind: Kind.Service
}

export const getService = (overrides?: DeepPartial<Service>) => mergeDeepWith(
    concat,
    DEFAULT_SERVICE_RESOURCE,
    overrides
)
