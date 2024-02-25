import { mergeDeepRight } from 'ramda'
import { DeepPartial } from 'lib/common'
import { AppProject, Application, ConfigMap, Deployment, HorizontalPodAutoscaler, Ingress, Kustomization, Service } from './specifications'
import {
    DEFAULT_APPLICATION_RESOURCE,
    DEFAULT_APP_PROJECT_RESOURCE,
    DEFAULT_CONFIG_MAP_RESOURCE,
    DEFAULT_DEPLOYMENT_RESOURCE,
    DEFAULT_HPA_RESOURCE,
    DEFAULT_INGRESS_RESOURCE,
    DEFAULT_KUSTOMIZATION_RESOURCE,
    DEFAULT_SERVICE_RESOURCE
} from './constants'

const override = <T extends object>(manifest: T, overrides?: DeepPartial<T>) => mergeDeepRight(manifest, overrides as T)

export const getAppProject = (overrides?: DeepPartial<AppProject>): AppProject => override(DEFAULT_APP_PROJECT_RESOURCE, overrides)
export const getApplication = (overrides?: DeepPartial<Application>): Application => override(DEFAULT_APPLICATION_RESOURCE, overrides)
export const getConfigMap = (overrides?: DeepPartial<ConfigMap>): ConfigMap => override(DEFAULT_CONFIG_MAP_RESOURCE, overrides)
export const getDeployment = (overrides?: DeepPartial<Deployment>): Deployment => override(DEFAULT_DEPLOYMENT_RESOURCE, overrides)
export const getService = (overrides?: DeepPartial<Service>): Service => override(DEFAULT_SERVICE_RESOURCE, overrides)
export const getIngress = (overrides?: DeepPartial<Ingress>): Ingress => override(DEFAULT_INGRESS_RESOURCE, overrides)
export const getHorizontalPodAutoscaler = (overrides?: DeepPartial<HorizontalPodAutoscaler>): HorizontalPodAutoscaler =>
    override(DEFAULT_HPA_RESOURCE, overrides)
export const getKustomization = (overrides?: DeepPartial<Kustomization>): Kustomization => override(DEFAULT_KUSTOMIZATION_RESOURCE, overrides)
