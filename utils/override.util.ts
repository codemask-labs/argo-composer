import { mergeDeepRight } from 'ramda'
import { DeepPartial } from '../types'

export const override = <T extends object>(source: T, overrides?: DeepPartial<T>) => mergeDeepRight(source, overrides || {})
