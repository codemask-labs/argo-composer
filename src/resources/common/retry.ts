import { Backoff } from './backoff'

export type Retry = {
    limit: number
    backoff: Backoff
}
