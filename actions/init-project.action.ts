import { input } from '@inquirer/prompts'

export const initProjectAction = async () => {
    const name = await input({ message: 'What name would you like to use for the project?' })
    const repoURL = await input({ message: 'What is the base URL of GitHub repository?' })
    const environments = await input({
        message: 'What will be the environment inside your cluster? Provide separated by `,`',
        default: 'dev,prod'
    })

    // const environments = await input({
    //     message: 'What will be the environment inside your cluster? Provide separated by `,`',
    //     default: 'dev,prod'
    // }).then(environments =>
    //     environments
    //         .toLowerCase()
    //         .split(',')
    //         .map(environment => environment.trim())
    // )

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
