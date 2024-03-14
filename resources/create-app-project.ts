import { DEFAULT_APP_PROJECT } from '../constants'
import { Destination } from '../types'
import { override } from '../utils'

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
