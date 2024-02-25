import { Automated } from './automated'
import { Retry } from './retry'

export type SyncPolicy = {
    automated: Automated
    syncOptions: Array<string>
    retry: Retry
}
