import { DEFAULT_KUSTOMIZATION } from '../constants'
import { DeepPartial } from '../types'
import { override } from '../utils'
import { Kustomization } from './types'

export const createKustomization = (overrides: DeepPartial<Kustomization>) => override(DEFAULT_KUSTOMIZATION, overrides)
