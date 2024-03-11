import { DEFAULT_APPLICATION } from '../../constants/default-application'
import { override } from '../../utils'

type CreateApplication = {
    name: string
    namespace: string
    repoURL: string
    project?: string
    path?: string
}

export const createApplication = (values: CreateApplication) => override(DEFAULT_APPLICATION, {
    metadata: {
        name: values.name
    },
    spec: {
        project: values.project,
        destination: {
            namespace: values.namespace
        },
        source: {
            repoURL: values.repoURL,
            path: values.path
        }
    }
})
