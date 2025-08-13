/* eslint-disable no-template-curly-in-string */
import { join } from 'node:path'
import { ensureDir } from 'fs-extra'
import { parse, stringify } from 'yaml'
import { uniq } from 'ramda'
import { ARGO_COMPOSER_DIR, DEFAULT_PROFILE } from '../constants/paths'
import { writeYamlFile } from '../utils'
import { createApplication, createConfigMap, createIngress, createHorizontalPodAutoscaler, createKustomization } from '../resources'
import { ConfigInput, ScaffoldOptions, TemplateFile } from './types'
import { DEFAULT_PRESET_CONFIG } from '../constants/init-values'

const listPlaceholders = (content: string) => {
    const regex = /\$\{\{\s*input\.([A-Z0-9_]+)\s*\}\}/g

    return uniq(Array.from(content.matchAll(regex)).map(placeholder => placeholder[1]))
}

const getPresetInputs = (presetConfigYaml: string) => {
    const cfg = parse(presetConfigYaml) as ConfigInput

    return uniq(cfg.inputs?.map(input => input.name) ?? [])
}

const validatePlaceholders = (templateFiles: Array<TemplateFile>, presetConfigYaml: string) => {
    const inputs = getPresetInputs(presetConfigYaml)

    templateFiles.forEach(file => {
        const placeholders = listPlaceholders(file.content)

        placeholders.forEach(placeholder => {
            if (!inputs.includes(placeholder)) {
                throw new Error(`Template placeholder not declared in preset inputs: ${placeholder} in ${file.path}`)
            }
        })
    })
}

export const scaffoldComposerFolder = async ({ rootDirectory, mainRepositoryUrl, environments }: ScaffoldOptions) => {
    const root = join(process.cwd(), rootDirectory, ARGO_COMPOSER_DIR)
    const templateDir = join(root, DEFAULT_PROFILE, 'presets', 'default', 'template')
    const baseDir = join(templateDir, 'base')
    const overlaysDir = join(templateDir, 'overlays')

    // create needed directories
    await Promise.all([ensureDir(baseDir), ensureDir(overlaysDir)])

    // File plans
    const mergedConfigPath = join(root, 'argo-composer.yaml')
    const mergedEffective = {
        version: 2,
        defaults: { profile: DEFAULT_PROFILE, preset: 'default' },
        profile: DEFAULT_PROFILE,
        metadata: { owner: '', team: '' },
        mainRepositoryUrl: typeof mainRepositoryUrl === 'string' ? mainRepositoryUrl : '',
    }

    const tplKustomizationPath = join(root, DEFAULT_PROFILE, 'presets', 'default', 'template', 'kustomization.yaml')
    const tplKustomizationObj = createKustomization({ resources: ['application.yaml'] })

    const tplApplicationPath = join(root, DEFAULT_PROFILE, 'presets', 'default', 'template', 'application.yaml')
    const tplApplicationObj = createApplication({
        name: '${{ input.APP_NAME }}-${{ input.ENV }}',
        namespace: '${{ input.NAMESPACE }}',
        repoURL: '${{ input.REPO_URL }}',
        project: 'default',
        path: './overlays/${{ input.ENV }}',
    })
    // App base files as objects
    const baseConfigMapObj = createConfigMap({ applicationName: '${{ input.APP_NAME }}', data: {} })
    const baseDeploymentObj = {
        apiVersion: 'apps/v1',
        kind: 'Deployment',
        metadata: { name: '${{ input.APP_NAME }}' },
        spec: {
            replicas: 1,
            selector: { matchLabels: { app: '${{ input.APP_NAME }}' } },
            template: {
                metadata: { labels: { app: '${{ input.APP_NAME }}' } },
                spec: {
                    containers: [
                        {
                            name: '${{ input.APP_NAME }}',
                            image: '${{ input.IMAGE_NAME }}',
                            ports: [{ containerPort: '${{ input.CONTAINER_PORT }}' }],
                        },
                    ],
                },
            },
        },
    }

    const baseServiceObj = {
        apiVersion: 'v1',
        kind: 'Service',
        metadata: { name: '${{ input.APP_NAME }}' },
        spec: {
            selector: { app: '${{ input.APP_NAME }}' },
            ports: [{ port: '${{ input.SERVICE_PORT }}', targetPort: '${{ input.CONTAINER_PORT }}' }],
        },
    }
    const baseIngressObj = createIngress({ applicationName: '${{ input.APP_NAME }}' })
    const baseHpaObj = createHorizontalPodAutoscaler({ applicationName: '${{ input.APP_NAME }}' })
    const baseKustomizationObj = createKustomization({
        resources: ['./configmap.yaml', './deployment.yaml', './service.yaml', './ingress.yaml', './hpa.yaml'],
    })

    // Overlays seed for each environment
    type YamlObject = Record<string, unknown>
    type YamlArray = Array<Record<string, unknown>>

    const overlayPlans: Array<{ path: string; obj: YamlObject | YamlArray }> = environments.flatMap((env: string) => {
        const envDir = join(overlaysDir, env)
        const kustomizationObj = createKustomization({
            resources: ['../../base'],
            // DeepPartial allows us to add patchesJson6902 here
            // eslint-disable-next-line @typescript-eslint/ban-ts-comment
            // @ts-ignore
            patchesJson6902: [
                {
                    target: { group: 'apps', version: 'v1', kind: 'Deployment', name: '${{ input.APP_NAME }}' },
                    path: './patches.yaml',
                },
            ],
        })

        const patchesArr: YamlArray = [{ op: 'replace', path: '/spec/template/spec/containers/0/image', value: '${{ input.IMAGE_NAME }}' }]
        return [
            { path: join(envDir, 'kustomization.yaml'), obj: kustomizationObj },
            { path: join(envDir, 'patches.yaml'), obj: patchesArr },
        ]
    })

    // Ensure each overlays/<env> directory exists
    await Promise.all(environments.map((env: string) => ensureDir(join(overlaysDir, env))))

    // Validate YAML config files parse by stringifying objects
    parse(stringify(mergedEffective))
    parse(stringify(DEFAULT_PRESET_CONFIG))

    // Validate placeholders declared across all template files
    validatePlaceholders(
        [
            { path: tplApplicationPath, content: stringify(tplApplicationObj) },
            { path: join(baseDir, 'configmap.yaml'), content: stringify(baseConfigMapObj) },
            { path: join(baseDir, 'deployment.yaml'), content: stringify(baseDeploymentObj) },
            { path: join(baseDir, 'service.yaml'), content: stringify(baseServiceObj) },
            { path: join(baseDir, 'ingress.yaml'), content: stringify(baseIngressObj) },
            { path: join(baseDir, 'hpa.yaml'), content: stringify(baseHpaObj) },
            { path: join(baseDir, 'kustomization.yaml'), content: stringify(baseKustomizationObj) },
            ...overlayPlans.map(p => ({ path: p.path, content: stringify(p.obj) })),
        ],
        stringify(DEFAULT_PRESET_CONFIG)
    )

    // Write files using YAML writer
    await Promise.all([
        writeYamlFile(mergedConfigPath, mergedEffective),
        writeYamlFile(join(root, DEFAULT_PROFILE, 'profile.yaml'), { name: DEFAULT_PROFILE, environments }),
        writeYamlFile(join(root, DEFAULT_PROFILE, 'presets', 'default', 'preset.yaml'), DEFAULT_PRESET_CONFIG),
        writeYamlFile(tplKustomizationPath, tplKustomizationObj),
        writeYamlFile(tplApplicationPath, tplApplicationObj),
        writeYamlFile(join(baseDir, 'configmap.yaml'), baseConfigMapObj),
        writeYamlFile(join(baseDir, 'deployment.yaml'), baseDeploymentObj),
        writeYamlFile(join(baseDir, 'service.yaml'), baseServiceObj),
        writeYamlFile(join(baseDir, 'ingress.yaml'), baseIngressObj),
        writeYamlFile(join(baseDir, 'hpa.yaml'), baseHpaObj),
        writeYamlFile(join(baseDir, 'kustomization.yaml'), baseKustomizationObj),
        ...overlayPlans.map(p => writeYamlFile(p.path, p.obj)),
    ])
}
