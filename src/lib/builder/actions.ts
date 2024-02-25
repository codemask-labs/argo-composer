export const tree = (pipeline: Array<any>) => {}

tree([
    root('argo-resources'),
    create('argo-composer.config.yaml', {
        name: 'argo-resources',
        repoUrl: 'https://github.com/codemaskinc/argocd-resources',
        environments: ['dev', 'prod']
    }),
    createApplication('root-app.yaml', {
        metadata: {
            name: 'root-app'
        },
        spec: {
            destination: {
                server: 'kubernetes.default.svc',
                namespace: 'argocd'
            }
        }
    }),
    createKustomization('projects/kustomization.yaml', {
        resources: []
    })
])
