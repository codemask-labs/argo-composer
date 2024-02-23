import { concat, mergeDeepWith } from 'ramda'
import { DeepPartial } from '../../types'

type Kustomization = {
    resources: Array<string>
}

const DEFAULT_KUSTOMIZATION_RESOURCE: Kustomization = {
    resources: []
}

export const getKustomization = (overrides?: DeepPartial<Kustomization>): Kustomization => mergeDeepWith(
    concat,
    DEFAULT_KUSTOMIZATION_RESOURCE,
    overrides
)
