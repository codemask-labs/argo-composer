export type Metadata = {
    name: string
    namespace: string
    finalizers?: Array<string>
    annotations?: Record<string, string>
}
