type CreateService = {
    applicationName: string
    servicePort: number
    containerPort: number
}

export const createService = ({ applicationName, ...options }: CreateService) => ({
    apiVersion: 'v1',
    kind: 'Service',
    metadata: {
        name: `${applicationName}-svc`,
        labels: {
            app: applicationName
        },
        spec: {
            selector: applicationName,
            ports: [{ port: options.servicePort, targetPort: options.containerPort }]
        }
    }
})
