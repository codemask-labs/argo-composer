type GetDeploymentPatches = {
    applicationName: string
    environment: string
    servicePort: number
    useHorizontalPodAutoscaler?: boolean
}

export const getDeploymentPatches = (values: GetDeploymentPatches) => {
    const { applicationName, environment, ...options } = values

    const path = {
        path: '/',
        pathType: 'Prefix',
        backend: {
            service: {
                name: `${applicationName}-svc`,
                port: {
                    number: options.servicePort
                }
            }
        }
    }

    const rule = {
        host: 'example.com',
        http: {
            paths: [path]
        }
    }

    const tls = {
        secretName: `${applicationName}-${environment}-tls`,
        hosts: ['example.com']
    }

    const hpa = {
        apiVersion: 'autoscaling/v2',
        kind: 'HorizontalPodAutoscaler',
        metadata: {
            name: `${applicationName}-hpa`
        },
        spec: {
            minReplicas: 1,
            maxReplicas: 3
        }
    }

    return [
        {
            apiVersion: 'v1',
            kind: 'ConfigMap',
            metadata: {
                name: `${applicationName}-cm`
            },
            data: {
                ENVIRONMENT: environment
            }
        },
        {
            apiVersion: 'apps/v1',
            kind: 'Deployment',
            metadata: {
                name: applicationName,
                labels: {
                    app: applicationName
                }
            },
            spec: {
                replicas: 1
            }
        },
        {
            apiVersion: 'networking.k8s.io/v1',
            kind: 'Ingress',
            metadata: {
                name: applicationName
            },
            spec: {
                tls: [tls],
                rules: [rule]
            }
        },
        ...(!options.useHorizontalPodAutoscaler ? [] : [hpa])
    ]
}
