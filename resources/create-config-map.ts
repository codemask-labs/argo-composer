type CreateConfigMap = {
    applicationName: string
    data: Record<string, string>
}

export const createConfigMap = (values: CreateConfigMap) => ({
    apiVersion: 'v1',
    kind: 'ConfigMap',
    metadata: {
        name: `${values.applicationName}-cm`
    },
    data: values.data
})
