import { join, resolve } from 'node:path'
import { existsSync } from 'node:fs'
import { checkbox, input } from '@inquirer/prompts'
import { StacklessError, command, param } from '@codemaskjs/node-cli-toolkit'
import { PROJECT_CONFIG_NAME } from 'lib/common'
import { ProjectConfig, createAppProject, writeYamlFile } from 'lib/utils'
import { getApplication, getKustomization } from 'resources/utils'
import { createArgocdImageUpdaterAddon, createCertManagerAddon, createIngressNginxAddon, createReflectorAddong } from './addons'

export const INIT_COMMAND = command('init', {
    short: 'i',
    params: [
        param('name', { type: String }),
        param('repo-url', { type: String }),
        param('env', { type: String, short: 'e', multiple: true })
    ],
    description: 'Initializes argo composer root directory',
    next: async params => {
        const name = params.name || await input({
            message: 'What root directory name would you like to use for the project?',
            default: 'argocd-resources'
        })

        const root = resolve(process.cwd(), name)

        if (existsSync(join(root, PROJECT_CONFIG_NAME))) {
            throw new StacklessError('Argo composer root directory already initialized!')
        }

        const repoURL = params['repo-url'] || await input({
            message: 'What is the base URL of GitHub repository?'
        })

        // eslint-disable-next-line @typescript-eslint/ban-types
        const environments = params.env as unknown as Array<string> || await input({
            message: 'What will be the environment inside your cluster? Provide separated by `,`',
            default: 'dev,prod'
        })
            .then(environments =>
                environments
                    .toLowerCase()
                    .split(',')
                    .map(environment => environment.trim())
            )

        const config: ProjectConfig = {
            repoURL,
            environments
        }

        const kustomization = getKustomization({
            resources: []
        })

        const rootApplication = getApplication({
            metadata: { name: 'root-app' },
            spec: {
                source: {
                    repoURL,
                    path: 'projects'
                },
                syncPolicy: {
                    syncOptions: [
                        'ApplyOutOfSyncOnly=true',
                        'PruneLast=true'
                    ]
                }
            }
        })

        await writeYamlFile(join(root, 'projects', 'kustomization.yaml'), kustomization)
        await writeYamlFile(join(root, 'root-app.yaml'), rootApplication)
        await writeYamlFile(join(root, PROJECT_CONFIG_NAME), config)

        const addons = await checkbox({
            message: 'Do you want to install any additional components?',
            choices: [
                { name: 'ingress-nginx', value: createIngressNginxAddon },
                { name: 'cert-manager', value: createCertManagerAddon },
                { name: 'reflector', value: createReflectorAddong },
                { name: 'argocd-image-updater', value: createArgocdImageUpdaterAddon }
            ]
        })

        console.log('addons:', addons)

        const addonsProjectName = addons.length
            ? await input({ message: 'What name would you like to use for addons?', default: 'infra' })
            : undefined

        if (addonsProjectName) {
            const project = await createAppProject(addonsProjectName, {
                root
            })

            await Promise.all(addons.map(handle => handle(project)))
        }
    }
})
