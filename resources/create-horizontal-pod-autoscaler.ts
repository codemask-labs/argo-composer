type CreateHorizontalPodAutoscaler = {
    applicationName: string
}

export const createHorizontalPodAutoscaler = (values: CreateHorizontalPodAutoscaler) => {
    const { applicationName } = values

    const metric = {
        type: 'Resource',
        resource: {
            name: 'cpu',
            target: {
                type: 'Utilization',
                averageUtilization: 60
            }
        }
    }

    const policy = {
        type: 'Pods',
        value: 1,
        periodSeconds: 300
    }

    return {
        apiVersion: 'autoscaling/v2',
        kind: 'HorizontalPodAutoscaler',
        metadata: {
            name: `${applicationName}-hpa`
        },
        spec: {
            minReplicas: 1,
            maxReplicas: 3,
            scaleTargetRef: {
                apiVersion: 'apps/v1',
                kind: 'Deployment',
                name: applicationName
            },
            metrics: [metric],
            behavior: {
                scaleDown: {
                    stabilizationWindowSeconds: 60,
                    policies: [policy]
                }
            }
        }
    }
}
