import { isNil, mergeDeepWith } from 'ramda'
import { DeepPartial } from '../types'

export const override = <T extends object>(source: T, overrides?: DeepPartial<T>): T => {
    const replacer = <V>(from: V, to: V): V => (isNil(to) ? from : to)

    // Constrain typings at the boundary; return strongly typed T
    const result = mergeDeepWith(replacer as (from: unknown, to: unknown) => unknown, source as unknown, (overrides ?? {}) as unknown) as T

    return result
}
