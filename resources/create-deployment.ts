type CreateDeployment = {
    applicationName: string
    imageName: string
    containerPort: number
    useHealthCheck?: boolean
    useSecurityContext?: boolean
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const createDeployment = (values: CreateDeployment): Record<any, any> => {
    const healthCheck = {
        livenessProbe: {
            httpGet: {
                path: '/health-check',
                port: values.containerPort
            },
            initialDelaySeconds: 20,
            timeoutSeconds: 3,
            successThreshold: 1,
            failureThreshold: 5,
            periodSeconds: 10
        },
        readinessProbe: {
            httpGet: {
                path: '/health-check',
                port: values.containerPort
            },
            initialDelaySeconds: 5,
            timeoutSeconds: 3,
            successThreshold: 1,
            failureThreshold: 5,
            periodSeconds: 5
        }
    }

    const securityContext = {
        securityContext: {
            runAsNonRoot: true,
            runAsUser: 1000,
            allowPrivilegeEscalation: false
        }
    }

    return {
        apiVersion: 'apps/v1',
        kind: 'Deployment',
        metadata: {
            name: values.applicationName,
            labels: {
                app: values.applicationName
            }
        },
        spec: {
            replicas: 1,
            revisionHistoryLimit: 1,
            progressDeadlineSeconds: 600,
            selector: {
                matchLabels: {
                    app: values.applicationName
                }
            },
            strategy: {
                type: 'RollingUpdate',
                rollingUpdate: {
                    maxUnavailable: 0,
                    maxSurge: '100%'
                }
            },
            template: {
                metadata: {
                    labels: {
                        app: values.applicationName
                    }
                },
                spec: {
                    restartPolicy: 'Always',
                    containers: [
                        {
                            name: values.applicationName,
                            image: values.imageName,
                            imagePullPolicy: 'IfNotPresent',
                            ports: [{ containerPort: values.containerPort }],
                            ...(!values.useHealthCheck ? {} : healthCheck),
                            ...(!values.useSecurityContext ? {} : securityContext)
                        }
                    ]
                }
            }
        }
    }
}
