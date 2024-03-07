import { Application } from '../types/resources/application'

const application: Application = {
    apiVersion: '',
    kind: 'Application',
    metadata: {
        name: 'ingress-nginx',
        namespace: 'ingress-nginx',
        finalizers: [],
        labels: {
            name: ''
        }
    },
    // eslint-disable-next-line @typescript-eslint/no-explicit-any
    spec: {} as any
}

export const INGRESS_NGINX_ADDON_RESOURCES = [
    { type: 'application', name: 'ingress-nginx', resource: application }
]
