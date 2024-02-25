import { Application } from 'resources/specifications'

type Context = {
    root: string
    files: Record<string, object | Array<object>>
    join: (...paths: Array<string>) => string
}

type Tap<T> = {
    tap: (context: T) => void | T | Promise<void | T>
}

// eslint-disable-next-line @typescript-eslint/no-explicit-any
const root = (path: string): Tap<Context> => ({
    tap: context => ({
        ...context,
        root: path
    })
})

const createApplication = (path: string, application: Application): Tap<Context> => ({
    tap: async context => {
        const root = context.join(path)

    }
})

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const actions = <Context extends object>(pipeline: Array<Action<Context, any>>) => {
    console.log('pipeline:', pipeline)
}

actions([
    root('argo-resources')
    // createYaml('argo-composer.config.yaml', {
    //     name: 'argo-resources',
    //     repoUrl: 'https://github.com/codemaskinc/argocd-resources',
    //     environments: ['dev', 'prod']
    // }),
    // createApplication('root-app.yaml', {
    //     metadata: {
    //         name: 'root-app'
    //     },
    //     spec: {
    //         destination: {
    //             server: 'kubernetes.default.svc',
    //             namespace: 'argocd'
    //         }
    //     }
    // }),
    // createKustomization('projects/kustomization.yaml', {
    //     resources: []
    // })
])
