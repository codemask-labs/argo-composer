import { concat, mergeDeepWith } from 'ramda'
import { Kind } from '../../enums'
import { DeepPartial } from '../../types'

export type ConfigMap = {
    apiVersion: string
    kind: Kind.ConfigMap
}

export const DEFAULT_CONFIG_MAP_RESOURCE: ConfigMap = {
    apiVersion: 'v1',
    kind: Kind.ConfigMap
}

export const getConfigMap = (overrides?: DeepPartial<ConfigMap>): ConfigMap => mergeDeepWith(
    concat,
    DEFAULT_CONFIG_MAP_RESOURCE,
    overrides
)
