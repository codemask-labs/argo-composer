type CreateIngress = {
    applicationName: string
}

export const createIngress = (values: CreateIngress) => {
    const { applicationName } = values

    return {
        apiVersion: 'networking.k8s.io/v1',
        kind: 'Ingress',
        metadata: {
            name: applicationName,
            annotations: {
                'nginx.ingress.kubernetes.io/proxy-body-size': '10m',
                'nginx.ingress.kubernetes.io/proxy-read-timeout': '3600',
                'nginx.ingress.kubernetes.io/proxy-send-timeout': '3600',
                'nginx.ingress.kubernetes.io/proxy-buffer-size': '256k',
                'nginx.ingress.kubernetes.io/proxy-next-upstream': 'error timeout http_502',
                'nginx.ingress.kubernetes.io/http2-max-header-size': '256k',
                'nginx.ingress.kubernetes.io/large-client-header-buffers': '256 256k',
                'nginx.ingress.kubernetes.io/client-header-buffer-size': '256k',
                'nginx.ingress.kubernetes.io/client-body-buffer-size': '5m',
                'nginx.ingress.kubernetes.io/enable-brotli': 'true',
                'nginx.ingress.kubernetes.io/brotli-level': '5',
                'nginx.ingress.kubernetes.io/use-gzip': 'true',
                'nginx.ingress.kubernetes.io/gzip-level': '4'
            }
        },
        spec: {
            ingressClassName: 'nginx'
        }
    }
}
