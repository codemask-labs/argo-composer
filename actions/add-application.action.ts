/* eslint-disable max-lines */
import { join } from 'node:path'
import { confirm, input, select } from '@inquirer/prompts'
import { Application, Kustomization, ProjectConfig } from '../types'
import { getDirectoryList, getProjectConfig, isDirectory, readYamlFile, writeYamlFile } from '../utils'
import { createApplication } from '../resources'

type ApplicationOptions = {
    config: ProjectConfig
    imageName: string
    projectName: string
    applicationName: string
    applicationDirectory: string
    containerPort: number
    servicePort: number
    useHorizontalPodAutoscaler: boolean
    useImageUpdater: boolean
    useHealthCheck: boolean
    useSecurityContext: boolean
}

const getApplicationDestination = async () => {
    const projects = getDirectoryList('projects')

    const applicationName = await input({
        message: 'What is the name of application?'
    })

    const projectName = await select({
        message: `To which project we are adding '${applicationName}' application?`,
        choices: projects.map(value => ({ value }))
    })

    const applicationDirectory = join(process.cwd(), 'projects', projectName, 'apps', applicationName)

    if (isDirectory(applicationDirectory)) {
        throw new Error(`Application '${applicationName}' already exists in '${projectName}'`)
    }

    return {
        applicationName,
        projectName
    }
}

export const getApplicationNamespace = (environment: string, applicationName: string, projectName: string) =>
    projectName === 'default' ? `${applicationName}-${environment}` : `${projectName}-${applicationName}-${environment}`

export const addApplicationsWithOverlay = async (options: ApplicationOptions): Promise<Array<Application>> => {
    const { projectName, applicationName, applicationDirectory, config } = options
    const imageUpdaterAnnotations: Record<string, string> = !options.useImageUpdater
        ? {}
        : {
              'argocd-image-updater.argoproj.io/image-list': `i=${options.imageName}`,
              'argocd-image-updater.argoproj.io/i.update-strategy': 'latest',
              'argocd-image-updater.argoproj.io/i.allow-tags': 'regexp:([a-z0-9]+)-([0-9]+)',
              'argocd-image-updater.argoproj.io/i.force-update': 'true',
              'argocd-image-updater.argoproj.io/git-branch': 'main',
              'argocd-image-updater.argoproj.io/write-back-method': 'git',
              'argocd-image-updater.argoproj.io/write-back-target': 'kustomization'
          }

    const applications = config.environments.map(environment =>
        createApplication({
            name: `${applicationName}-${environment}`,
            namespace: getApplicationNamespace(environment, applicationName, projectName),
            project: projectName,
            repoURL: config.repoURL,
            path: `${applicationDirectory}/overlays/${environment}`,
            annotations: imageUpdaterAnnotations
        })
    )

    if (options.useHorizontalPodAutoscaler) {
        const metric = {
            type: 'Resource',
            resource: {
                name: 'cpu',
                target: {
                    type: 'Utilization',
                    averageUtilization: 60
                }
            }
        }

        const policy = {
            type: 'Pods',
            value: 1,
            periodSeconds: 300
        }

        await writeYamlFile(`${applicationDirectory}/base/hpa.yaml`, {
            apiVersion: 'autoscaling/v2',
            kind: 'HorizontalPodAutoscaler',
            metadata: {
                name: `${applicationName}-hpa`
            },
            spec: {
                minReplicas: 1,
                maxReplicas: 3,
                scaleTargetRef: {
                    apiVersion: 'apps/v1',
                    kind: 'Deployment',
                    name: applicationName
                },
                metrics: [metric],
                behavior: {
                    scaleDown: {
                        stabilizationWindowSeconds: 60,
                        policies: [policy]
                    }
                }
            }
        })
    }

    await writeYamlFile(`${applicationDirectory}/base/configmap.yaml`, {
        apiVersion: 'v1',
        kind: 'ConfigMap',
        metadata: {
            name: `${applicationName}-cm`
        },
        data: {}
    })

    await writeYamlFile(`${applicationDirectory}/base/deployment.yaml`, {
        apiVersion: 'apps/v1',
        kind: 'Deployment',
        metadata: {
            name: applicationName,
            labels: {
                app: applicationName
            }
        },
        spec: {
            progressDeadlineSeconds: 600,
            revisionHistoryLimit: 0,
            selector: {
                matchLabels: {
                    app: applicationName
                }
            },
            strategy: {
                type: 'RollingUpdate',
                rollingUpdate: {
                    maxUnavailable: 0,
                    maxSurge: '100%'
                }
            },
            template: {
                metadata: {
                    labels: {
                        app: applicationName
                    }
                },
                spec: {
                    restartPolicy: 'Always',
                    containers: [
                        {
                            name: applicationName,
                            image: options.imageName,
                            imagePullPolicy: 'IfNotPresent',
                            ports: [
                                {
                                    containerPort: 3000,
                                    name: 'http-3000'
                                }
                            ],
                            ...(!options.useHealthCheck
                                ? {}
                                : {
                                      livenessProbe: {
                                          httpGet: {
                                              path: '/health-check',
                                              port: 3000
                                          },
                                          initialDelaySeconds: 20,
                                          timeoutSeconds: 3,
                                          successThreshold: 1,
                                          failureThreshold: 5,
                                          periodSeconds: 10
                                      },
                                      readinessProbe: {
                                          httpGet: {
                                              path: '/health-check',
                                              port: 3000
                                          },
                                          initialDelaySeconds: 5,
                                          timeoutSeconds: 3,
                                          successThreshold: 1,
                                          failureThreshold: 5,
                                          periodSeconds: 5
                                      }
                                  }),
                            ...(!options.useSecurityContext
                                ? {}
                                : {
                                      securityContext: {
                                          runAsNonRoot: true,
                                          runAsUser: 1000,
                                          allowPrivilegeEscalation: false
                                      }
                                  })
                        }
                    ]
                }
            }
        }
    })

    await writeYamlFile(`${applicationDirectory}/base/service.yaml`, {
        apiVersion: 'v1',
        kind: 'Service',
        metadata: {
            name: `${applicationName}-svc`,
            labels: {
                app: applicationName
            },
            spec: {
                selector: applicationName,
                ports: [{ port: options.servicePort, targetPort: options.containerPort }]
            }
        }
    })
    await writeYamlFile(`${applicationDirectory}/base/ingress.yaml`, {
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
                // todo: figure out how to write raw strings
                // 'nginx.ingress.kubernetes.io/configuration-snippet': `
                //     more_set_headers "X-Content-Type-Options: nosniff";
                //     more_set_headers "X-Frame-Options: SAMEORIGIN";
                //     more_set_headers "X-Xss-Protection: 1; mode=block";
                //     more_set_headers "Content-Security-Policy: upgrade-insecure-requests";
                //     more_set_headers "Referrer-Policy: no-referrer-when-downgrade";
                // `
            }
        },
        spec: {
            ingressClassName: 'nginx'
        }
    })

    await writeYamlFile(`${applicationDirectory}/base/kustomization.yaml`, {
        resources: [
            './configmap.yaml',
            './deployment.yaml',
            './service.yaml',
            './ingress.yaml',
            ...(options.useHorizontalPodAutoscaler ? ['./hpa.yaml'] : [])
        ]
    })

    await Promise.all(
        config.environments.map(async environment => {
            const path = {
                path: '/',
                pathType: 'Prefix',
                backend: {
                    service: {
                        name: `${applicationName}-svc`,
                        port: {
                            number: options.servicePort
                        }
                    }
                }
            }

            const rule = {
                host: 'example.com',
                http: {
                    paths: [path]
                }
            }

            const tls = {
                secretName: `${applicationName}-${environment}-tls`,
                hosts: ['example.com']
            }

            await writeYamlFile(`${applicationDirectory}/overlays/${environment}/patches.yaml`, [
                {
                    apiVersion: 'v1',
                    kind: 'ConfigMap',
                    metadata: {
                        name: `${applicationName}-cm`
                    },
                    data: {
                        ENVIRONMENT: environment
                    }
                },
                {
                    apiVersion: 'apps/v1',
                    kind: 'Deployment',
                    metadata: {
                        name: applicationName,
                        labels: {
                            app: applicationName
                        }
                    },
                    spec: {
                        replicas: 1,
                        revisionHistoryLimit: 1
                    }
                },
                {
                    apiVersion: 'networking.k8s.io/v1',
                    kind: 'Ingress',
                    metadata: {
                        name: applicationName
                    },
                    spec: {
                        tls: [tls],
                        rules: [rule]
                    }
                },
                ...(!options.useHorizontalPodAutoscaler
                    ? []
                    : [
                          {
                              apiVersion: 'autoscaling/v2',
                              kind: 'HorizontalPodAutoscaler',
                              metadata: {
                                  name: `${applicationName}-hpa`
                              },
                              spec: {
                                  minReplicas: 1,
                                  maxReplicas: 3
                              }
                          }
                      ])
            ])

            await writeYamlFile(`${applicationDirectory}/overlays/${environment}/kustomization.yaml`, {
                resources: ['../../base'],
                // images: [{ name: imageUrlOrName }],
                patches: [{ path: './patches.yaml' }]
            })
        })
    )

    return applications
}

export const addApplicationWithResources = async (options: ApplicationOptions): Promise<Application> => {
    const { projectName, applicationName, applicationDirectory, config } = options

    const imageUpdaterAnnotations: Record<string, string> = !options.useImageUpdater
        ? {}
        : {
              'argocd-image-updater.argoproj.io/image-list': `i=${options.imageName}`,
              'argocd-image-updater.argoproj.io/i.update-strategy': 'latest',
              'argocd-image-updater.argoproj.io/i.allow-tags': 'regexp:([a-z0-9]+)-([0-9]+)',
              'argocd-image-updater.argoproj.io/i.force-update': 'true',
              'argocd-image-updater.argoproj.io/git-branch': 'main',
              'argocd-image-updater.argoproj.io/write-back-method': 'git',
              'argocd-image-updater.argoproj.io/write-back-target': 'kustomization'
          }

    const applicationPath = `${applicationDirectory}/resources`
    const application = createApplication({
        name: applicationName,
        namespace: applicationName,
        project: projectName,
        repoURL: config.repoURL,
        path: applicationPath,
        annotations: imageUpdaterAnnotations
    })

    if (options.useHorizontalPodAutoscaler) {
        const metric = {
            type: 'Resource',
            resource: {
                name: 'cpu',
                target: {
                    type: 'Utilization',
                    averageUtilization: 60
                }
            }
        }

        const policy = {
            type: 'Pods',
            value: 1,
            periodSeconds: 300
        }

        await writeYamlFile(`${applicationPath}/hpa.yaml`, {
            apiVersion: 'autoscaling/v2',
            kind: 'HorizontalPodAutoscaler',
            metadata: {
                name: `${applicationName}-hpa`
            },
            spec: {
                scaleTargetRef: {
                    apiVersion: 'apps/v1',
                    kind: 'Deployment',
                    name: applicationName
                },
                metrics: [metric],
                behavior: {
                    scaleDown: {
                        stabilizationWindowSeconds: 60,
                        policies: [policy]
                    }
                }
            }
        })
    }

    await writeYamlFile(`${applicationPath}/configmap.yaml`, {
        apiVersion: 'v1',
        kind: 'ConfigMap',
        metadata: {
            name: `${applicationName}-cm`
        },
        data: {}
    })

    await writeYamlFile(`${applicationPath}/deployment.yaml`, {
        apiVersion: 'apps/v1',
        kind: 'Deployment',
        metadata: {
            name: applicationName,
            labels: {
                app: applicationName
            }
        },
        spec: {
            progressDeadlineSeconds: 600,
            revisionHistoryLimit: 0,
            selector: {
                matchLabels: {
                    app: applicationName
                }
            },
            strategy: {
                type: 'RollingUpdate',
                rollingUpdate: {
                    maxUnavailable: 0,
                    maxSurge: '100%'
                }
            },
            template: {
                metadata: {
                    labels: {
                        app: applicationName
                    }
                },
                spec: {
                    restartPolicy: 'Always',
                    containers: [
                        {
                            name: applicationName,
                            image: options.imageName,
                            imagePullPolicy: 'IfNotPresent',
                            ports: [
                                {
                                    containerPort: 3000,
                                    name: 'http-3000'
                                }
                            ],
                            ...(!options.useHealthCheck
                                ? {}
                                : {
                                      livenessProbe: {
                                          httpGet: {
                                              path: '/health-check',
                                              port: 3000
                                          },
                                          initialDelaySeconds: 20,
                                          timeoutSeconds: 3,
                                          successThreshold: 1,
                                          failureThreshold: 5,
                                          periodSeconds: 10
                                      },
                                      readinessProbe: {
                                          httpGet: {
                                              path: '/health-check',
                                              port: 3000
                                          },
                                          initialDelaySeconds: 5,
                                          timeoutSeconds: 3,
                                          successThreshold: 1,
                                          failureThreshold: 5,
                                          periodSeconds: 5
                                      }
                                  }),
                            ...(!options.useSecurityContext
                                ? {}
                                : {
                                      securityContext: {
                                          runAsNonRoot: true,
                                          runAsUser: 1000,
                                          allowPrivilegeEscalation: false
                                      }
                                  })
                        }
                    ]
                }
            }
        }
    })

    await writeYamlFile(`${applicationPath}/service.yaml`, {
        apiVersion: 'v1',
        kind: 'Service',
        metadata: {
            name: `${applicationName}-svc`,
            labels: {
                app: applicationName
            },
            spec: {
                selector: applicationName,
                ports: [{ port: options.servicePort, targetPort: options.containerPort }]
            }
        }
    })

    const path = {
        path: '/',
        pathType: 'Prefix',
        backend: {
            service: {
                name: `${applicationName}-svc`,
                port: {
                    number: options.servicePort
                }
            }
        }
    }

    const rule = {
        host: 'example.com',
        http: {
            paths: [path]
        }
    }

    const tls = {
        secretName: `${applicationName}-tls`,
        hosts: ['example.com']
    }

    await writeYamlFile(`${applicationPath}/ingress.yaml`, {
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
                'nginx.ingress.kubernetes.io/gzip-level': '4',
                'nginx.ingress.kubernetes.io/configuration-snippet': `
                    more_set_headers "X-Content-Type-Options: nosniff";
                    more_set_headers "X-Frame-Options: SAMEORIGIN";
                    more_set_headers "X-Xss-Protection: 1; mode=block";
                    more_set_headers "Content-Security-Policy: upgrade-insecure-requests";
                    more_set_headers "Referrer-Policy: no-referrer-when-downgrade";
                `
            }
        },
        spec: {
            ingressClassName: 'nginx',
            tls: [tls],
            rules: [rule]
        }
    })

    await writeYamlFile(`${applicationPath}/kustomization.yaml`, {
        resources: [
            './configmap.yaml',
            './deployment.yaml',
            './service.yaml',
            './ingress.yaml',
            ...(options.useHorizontalPodAutoscaler ? ['./hpa.yaml'] : [])
        ]
    })

    return application
}

export const addApplicationAction = async () => {
    const config = getProjectConfig()
    const { applicationName, projectName } = await getApplicationDestination()

    const imageName = await input({
        message: 'What is the image name (for example: your-registry.com/your-app, nginx:latest etc.)?'
    })

    const containerPort = await input({
        message: 'What is the container port?'
    })

    const servicePort = await input({
        message: 'What is the service port?',
        default: containerPort
    })

    const useOverlays = await confirm({
        message: 'Use overlays (multiple environments)?',
        default: true
    })

    const useImageUpdater = await confirm({
        message: 'Use image-updater?',
        default: true
    })

    const useHorizontalPodAutoscaler = await confirm({
        message: 'Use HPA (Horizontal Pod Autoscaler)?',
        default: true
    })

    const useHealthCheck = await confirm({
        message: 'Use health check?',
        default: true
    })

    const useSecurityContext = await confirm({
        message: 'Use security context?',
        default: true
    })

    const applicationDirectory = `projects/${projectName}/apps/${applicationName}`
    const options: ApplicationOptions = {
        config,
        imageName,
        projectName,
        applicationName,
        applicationDirectory,
        containerPort: parseInt(containerPort, 10),
        servicePort: parseInt(servicePort, 10),
        useHorizontalPodAutoscaler,
        useImageUpdater,
        useHealthCheck,
        useSecurityContext
    }

    const applicationResources = useOverlays ? await addApplicationsWithOverlay(options) : await addApplicationWithResources(options)

    await writeYamlFile(`${applicationDirectory}/application.yaml`, applicationResources)
    await writeYamlFile(`${applicationDirectory}/kustomization.yaml`, {
        resources: ['./application.yaml']
    })

    const applicationsKustomizationPath = `projects/${projectName}/apps/kustomization.yaml`
    const applicationsKustomization = await readYamlFile<Kustomization>(applicationsKustomizationPath)

    await writeYamlFile(applicationsKustomizationPath, {
        resources: [...applicationsKustomization.resources, `./${applicationName}`]
    })
}
