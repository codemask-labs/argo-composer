import { DEFAULT_APP_PROJECT } from '../../constants'
import { override } from '../../utils'
import { Destination } from '../types'

type CreateAppProject = {
    name: string
    destinations?: Array<Destination>
    sourceRepos?: Array<string>
}

export const createAppProject = (values: CreateAppProject) =>
    override(DEFAULT_APP_PROJECT, {
        metadata: {
            name: values.name
        },
        spec: {
            sourceRepos: values.sourceRepos,
            destinations: values.destinations
        }
    })
