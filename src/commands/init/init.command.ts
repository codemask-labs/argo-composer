import { join } from 'node:path'
import { input } from '@inquirer/prompts'
import { command } from '@codemaskjs/node-cli-toolkit'
import { PROJECT_CONFIG_NAME } from 'lib/common'
import { writeYamlFile } from 'lib/utils'
import { getApplication, getKustomization } from 'lib/resources'

export const INIT_COMMAND = command('init', {
    short: 'i',
    description: 'Initializes argo composer root directory',
    next: async () => {
        const name = await input({
            message: 'What root directory name would you like to use for the project?',
            default: 'argocd-resources'
        })

        const repoURL = await input({
            message: 'What is the base URL of GitHub repository?'
        })

        const environments = await input({
            message: 'What will be the environment inside your cluster? Provide separated by `,`',
            default: 'dev,prod'
        })
            .then(environments =>
                environments
                    .toLowerCase()
                    .split(',')
                    .map(environment => environment.trim())
            )

        const root = join(process.cwd(), name)
        const config = {
            name,
            repoURL,
            environments
        }

        const application = getApplication({
            metadata: { name: 'root-app' },
            spec: {
                source: {
                    repoURL,
                    path: 'projects'
                }
            }
        })

        await writeYamlFile(join(root, 'projects', 'default', 'apps', 'kustomization.yaml'), getKustomization({
            resources: []
        }))
        await writeYamlFile(join(root, 'projects', 'default', 'kustomization.yaml'), getKustomization({
            resources: [ 'apps' ]
        }))
        await writeYamlFile(join(root, 'projects', 'kustomization.yaml'), getKustomization({
            resources: [ 'default' ]
        }))
        await writeYamlFile(join(root, PROJECT_CONFIG_NAME), config)
        await writeYamlFile(join(root, 'root-app.yaml'), application)

        // const additionalApps = await checkbox({
        //     message: 'Do you want to install any additional components?',
        //     choices: [
        //         { name: 'ingress-nginx', value: 'ingress-nginx' },
        //         { name: 'cert-manager', value: 'cert-manager' },
        //         { name: 'reflector', value: 'reflector' },
        //         { name: 'argocd-image-updater', value: 'argocd-image-updater' }
        //     ]
        // })

        // const addonsProjectName = additionalApps.length
        //     ? await input({ message: 'What name would you like to use for addons?', default: 'infra' })
        //     : undefined

        // const addonsProject = additionalApps.length
        //     ? [mergeWith(apply(url('./addons/addons-project'), [template({ repoURL, name, addonsProjectName, ...strings })]))]
        //     : []

        // const addons = addonsProjectName ? additionalApps.map(addon => addonTemplate(addon, { name, repoURL }, addonsProjectName)) : []

        // const templateSource = apply(url('./files'), [template({ name, repoURL, environments, ...strings })])

        // return chain([
        //     mergeWith(templateSource),
        //     ...addonsProject,
        //     ...addons,
        //     updateAddonsKustomization(name, additionalApps, addonsProjectName),
        //     updateProjectKustomization(name, addonsProjectName)
        // ])
    }
})
