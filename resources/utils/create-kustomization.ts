import { DeepPartial } from '../../types'
import { Kustomization } from '../types'
import { DEFAULT_KUSTOMIZATION } from '../../constants'
import { override } from '../../utils'

export const createKustomization = (overrides: DeepPartial<Kustomization>) => override(DEFAULT_KUSTOMIZATION, overrides)
