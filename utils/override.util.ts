import { isNil, mergeDeepWith } from 'ramda'
import { DeepPartial } from '../types'

export const override = <T extends object>(source: T, overrides?: DeepPartial<T>) =>
    mergeDeepWith(
        // eslint-disable-next-line @typescript-eslint/no-explicit-any
        (from: any, to: any) => (isNil(to) ? from : to),
        source,
        overrides || {}
    )
