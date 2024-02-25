export type Probe = {
    httpGet: {
        path: 'string'
        port: number
    }
    initialDelaySeconds: number
    timeoutSeconds: 3
    successThreshold: number
    failureThreshold: number
    periodSeconds: number
}
