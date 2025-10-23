use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::resources::common::ApplicationDestination;

/// Application is a definition of Application resource in ArgoCD.
///
/// ArgoCD Application CRD represents a deployed application instance in an environment.
/// It describes the source (Git repository or Helm chart) and destination (Kubernetes cluster and namespace)
/// for deploying Kubernetes manifests.
///
/// # API Version
/// `argoproj.io/v1alpha1`
///
/// # Example
/// ```yaml
/// apiVersion: argoproj.io/v1alpha1
/// kind: Application
/// metadata:
///   name: guestbook
///   namespace: argocd
/// spec:
///   project: default
///   source:
///     repoURL: https://github.com/argoproj/argocd-example-apps.git
///     targetRevision: HEAD
///     path: guestbook
///   destination:
///     server: https://kubernetes.default.svc
///     namespace: guestbook
///   syncPolicy:
///     automated:
///       prune: true
///       selfHeal: true
/// ```
///
/// # Reference
/// See [ArgoCD Application Spec](https://argo-cd.readthedocs.io/en/stable/operator-manual/application.yaml)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Application {
    /// APIVersion defines the versioned schema of this representation of an object.
    /// Must be `argoproj.io/v1alpha1`
    #[serde(rename = "apiVersion")]
    pub api_version: String,

    /// Kind is a string value representing the REST resource this object represents.
    /// Must be `Application`
    #[serde(rename = "kind")]
    pub kind: String,

    /// Standard object's metadata.
    /// More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    /// Status contains information about the application's current sync and health status.
    /// This field is managed by ArgoCD and should not be set directly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ApplicationStatus>,

    /// Spec defines the desired state of the Application.
    pub spec: ApplicationSpec,
}

impl Default for Application {
    fn default() -> Self {
        Application {
            api_version: "argoproj.io/v1alpha1".to_string(),
            kind: "Application".to_string(),
            metadata: None,
            status: None,
            spec: ApplicationSpec::default(),
        }
    }
}

/// ApplicationSpec represents the desired state of an Application.
///
/// The specification defines where the application manifests come from (source),
/// where they should be deployed (destination), and how they should be synced (syncPolicy).
///
/// # Fields Overview
/// - `project`: References an AppProject that provides governance and resource limits
/// - `source`/`sources`: Defines Git repositories or Helm charts containing the application manifests
/// - `destination`: Specifies the target Kubernetes cluster and namespace
/// - `syncPolicy`: Controls automated sync behavior, retry strategies, and sync options
/// - `ignoreDifferences`: Configures fields to ignore during comparison
/// - `info`: Additional metadata displayed in the UI
///
/// # Multi-Source Applications
/// Applications can use either `source` (single source) or `sources` (multiple sources).
/// Multi-source is the preferred approach for modern applications that need to combine
/// multiple sources (e.g., a Helm chart with additional manifests).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApplicationSpec {
    /// Project is the name of the AppProject this application belongs to.
    /// The AppProject provides governance, resource whitelist/blacklist, and destination restrictions.
    /// Defaults to 'default' if not specified.
    ///
    /// # Example
    /// ```yaml
    /// project: my-team
    /// ```
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub project: String,

    /// Source is a reference to the location of the application's manifests or Helm chart.
    /// Use this for single-source applications (legacy mode).
    /// For multi-source applications, use `sources` instead.
    ///
    /// # Example
    /// ```yaml
    /// source:
    ///   repoURL: https://github.com/argoproj/argocd-example-apps
    ///   targetRevision: HEAD
    ///   path: guestbook
    /// ```
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ApplicationSource>,

    /// Sources is a list of source references for multi-source applications.
    /// This is the preferred approach for applications that combine multiple sources
    /// (e.g., Helm charts with value overrides from Git, or multiple Kustomize bases).
    ///
    /// # Example
    /// ```yaml
    /// sources:
    ///   - repoURL: https://helm.example.com
    ///     chart: my-app
    ///     targetRevision: 1.0.0
    ///   - repoURL: https://github.com/myorg/values
    ///     path: production/values.yaml
    ///     targetRevision: main
    /// ```
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<ApplicationSource>,

    /// Destination specifies the target Kubernetes cluster and namespace for deployment.
    ///
    /// # Example
    /// ```yaml
    /// destination:
    ///   server: https://kubernetes.default.svc
    ///   namespace: production
    /// ```
    #[serde(default)]
    pub destination: ApplicationDestination,

    /// SyncPolicy controls how the application will be synced with the source repository.
    /// It includes automated sync settings, retry strategies, and sync options.
    ///
    /// # Example
    /// ```yaml
    /// syncPolicy:
    ///   automated:
    ///     prune: true
    ///     selfHeal: true
    ///   syncOptions:
    ///     - CreateNamespace=true
    /// ```
    #[serde(rename = "syncPolicy", skip_serializing_if = "Option::is_none")]
    pub sync_policy: Option<SyncPolicy>,

    /// RevisionHistoryLimit limits the number of items kept in the application's revision history.
    /// This is used for garbage collection and limits how many revisions are kept in status.history.
    /// Defaults to 10 if not specified.
    ///
    /// # Example
    /// ```yaml
    /// revisionHistoryLimit: 5
    /// ```
    #[serde(
        rename = "revisionHistoryLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub revision_history_limit: Option<i64>,

    /// IgnoreDifferences specifies resource fields which should be ignored during comparison.
    /// This is useful for ignoring fields that are expected to differ (e.g., replica counts
    /// managed by HPA, or fields modified by other controllers).
    ///
    /// # Example
    /// ```yaml
    /// ignoreDifferences:
    ///   - group: apps
    ///     kind: Deployment
    ///     jsonPointers:
    ///       - /spec/replicas
    /// ```
    #[serde(
        rename = "ignoreDifferences",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ignore_differences: Vec<ResourceIgnoreDifference>,

    /// Info contains a list of information items displayed in the ArgoCD UI for this application.
    /// This is useful for adding operational metadata, links, or documentation references.
    ///
    /// # Example
    /// ```yaml
    /// info:
    ///   - name: Documentation
    ///     value: https://docs.example.com/myapp
    ///   - name: Slack Channel
    ///     value: '#team-platform'
    /// ```
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub info: Vec<InfoItem>,

    /// SourceRef is an optional reference to another Application's source.
    /// This is an advanced feature rarely used in typical YAML configurations.
    #[serde(rename = "sourceRef", skip_serializing_if = "Option::is_none")]
    pub source_ref: Option<ApplicationSourceRef>,
}

/// ApplicationSource contains information about the source of application manifests.
///
/// Sources can be:
/// - Git repositories containing raw Kubernetes manifests
/// - Git repositories with Kustomize overlays
/// - Git repositories with Helm charts
/// - Helm chart repositories
/// - Git repositories with Jsonnet files
///
/// # Source Types
/// - **Plain Directory**: Set `repoURL` and `path`
/// - **Kustomize**: Set `repoURL`, `path`, and `kustomize` options
/// - **Helm from Git**: Set `repoURL`, `path`, and `helm` options
/// - **Helm from Registry**: Set `repoURL` (Helm registry), `chart`, and `helm` options
/// - **Jsonnet**: Set `repoURL`, `path`, and configure `directory.jsonnet`
/// - **Custom Plugin**: Set `repoURL`, `path`, and `plugin`
///
/// # Example - Git with Kustomize
/// ```yaml
/// source:
///   repoURL: https://github.com/myorg/myapp
///   targetRevision: v1.2.3
///   path: overlays/production
///   kustomize:
///     images:
///       - myapp=myapp:v1.2.3
/// ```
///
/// # Example - Helm from Registry
/// ```yaml
/// source:
///   repoURL: https://charts.example.com
///   chart: my-application
///   targetRevision: 2.1.0
///   helm:
///     values: |
///       replicas: 3
///       image:
///         tag: v2.1.0
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApplicationSource {
    /// RepoURL is the URL to the repository (Git or Helm) containing the application manifests.
    ///
    /// # Git Examples
    /// - `https://github.com/argoproj/argocd-example-apps`
    /// - `git@github.com:argoproj/argocd-example-apps.git`
    ///
    /// # Helm Examples
    /// - `https://charts.helm.sh/stable`
    /// - `oci://ghcr.io/myorg/helm-charts`
    #[serde(rename = "repoURL", default, skip_serializing_if = "String::is_empty")]
    pub repo_url: String,

    /// Path is the directory path within the Git repository where the application manifests are located.
    /// Only used for Git sources, not Helm chart repositories.
    ///
    /// # Examples
    /// - `guestbook`
    /// - `apps/production/myapp`
    /// - `kustomize/overlays/staging`
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String,

    /// TargetRevision defines the revision of the source to sync the application to.
    /// For Git sources: branch name, tag, or commit SHA
    /// For Helm charts: chart version or semver range
    ///
    /// # Git Examples
    /// - `HEAD` - latest commit on default branch
    /// - `main` - branch name
    /// - `v1.2.3` - tag
    /// - `abc123` - commit SHA
    ///
    /// # Helm Examples
    /// - `1.2.3` - exact version
    /// - `~1.2.0` - patch releases (>= 1.2.0, < 1.3.0)
    /// - `^1.2.0` - minor releases (>= 1.2.0, < 2.0.0)
    #[serde(
        rename = "targetRevision",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub target_revision: String,

    /// Chart is the Helm chart name. Required when using a Helm chart repository.
    /// Do not use with `path` (Git-based Helm charts use `path` instead).
    ///
    /// # Example
    /// ```yaml
    /// chart: nginx
    /// ```
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub chart: String,

    /// Kustomize holds Kustomize-specific options for rendering manifests.
    /// Only applicable when the source contains a kustomization.yaml file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kustomize: Option<KustomizeOptions>,

    /// Helm holds Helm-specific options for rendering charts.
    /// Applicable for both Git-based Helm charts and Helm repositories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub helm: Option<HelmOptions>,

    /// Directory holds options for applications defined as plain Kubernetes manifests.
    /// Includes recursion, filtering, and Jsonnet support.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory: Option<DirectoryOptions>,

    /// Plugin holds Config Management Plugin (CMP) specific options.
    /// Used for custom tool integrations beyond Helm, Kustomize, and plain manifests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin: Option<PluginOptions>,

    /// Ref is a symbolic reference name for this source in multi-source applications.
    /// Used when one source needs to reference values from another source.
    ///
    /// # Example
    /// ```yaml
    /// sources:
    ///   - repoURL: https://github.com/myorg/values
    ///     ref: values
    ///   - repoURL: https://charts.example.com
    ///     chart: myapp
    ///     helm:
    ///       valueFiles:
    ///         - $values/production.yaml
    /// ```
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub r#ref: String,
}

/// SyncPolicy controls when and how a sync will be performed.
////// Defines automated sync behavior, retry strategies, and various sync options
/// that modify how ArgoCD applies resources to the cluster.
///
/// # Example
/// ```yaml
/// syncPolicy:
///   automated:
///     prune: true
///     selfHeal: true
///   syncOptions:
///     - CreateNamespace=true
///     - PruneLast=true
///   retry:
///     limit: 5
///     backoff:
///       duration: 5s
///       factor: 2
///       maxDuration: 3m
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SyncPolicy {
    /// Automated controls the automated sync behavior.
    /// When set, ArgoCD will automatically sync the application when it detects drift.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated: Option<SyncPolicyAutomated>,

    /// SyncOptions provide additional configuration for the sync operation.
    ///
    /// # Common Options
    /// - `CreateNamespace=true` - Create namespace if it doesn't exist
    /// - `PruneLast=true` - Prune resources after all other operations complete
    /// - `PrunePropagationPolicy=foreground` - Set propagation policy for pruning
    /// - `ApplyOutOfSyncOnly=true` - Only sync resources that are out-of-sync
    /// - `Replace=true` - Use kubectl replace instead of apply
    /// - `ServerSideApply=true` - Use server-side apply
    /// - `SkipDryRunOnMissingResource=true` - Skip dry-run if resource is missing
    /// - `RespectIgnoreDifferences=true` - Respect ignoreDifferences during syncs
    #[serde(rename = "syncOptions", default, skip_serializing_if = "Vec::is_empty")]
    pub sync_options: Vec<String>,

    /// Retry controls the strategy to apply if a sync fails.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<RetryStrategy>,

    /// ManagedNamespaceMetadata controls metadata applied to the target namespace.
    /// Useful for adding labels or annotations to namespaces managed by ArgoCD.
    ///
    /// # Example
    /// ```yaml
    /// managedNamespaceMetadata:
    ///   labels:
    ///     team: platform
    ///     environment: production
    /// ```
    #[serde(
        rename = "managedNamespaceMetadata",
        skip_serializing_if = "Option::is_none"
    )]
    pub managed_namespace_metadata: Option<ManagedNamespaceMetadata>,
}

/// SyncPolicyAutomated controls the automated sync settings.
///
/// When automated sync is enabled, ArgoCD will automatically sync the application
/// when it detects the application is out of sync with the source repository.
///
/// # Example
/// ```yaml
/// automated:
///   prune: true
///   selfHeal: true
///   allowEmpty: false
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SyncPolicyAutomated {
    /// Prune specifies whether to delete resources from the cluster that are not defined in Git.
    /// When true, resources that exist in the cluster but not in Git will be deleted during sync.
    /// Default: false
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub prune: Option<bool>,

    /// SelfHeal specifies whether to force a sync when the application is not in-sync.
    /// When true, ArgoCD will automatically resync when it detects drift between Git and cluster.
    /// This overrides any manual changes made to resources in the cluster.
    /// Default: false
    #[serde(rename = "selfHeal", default, skip_serializing_if = "Option::is_none")]
    pub self_heal: Option<bool>,

    /// AllowEmpty allows apps with zero live resources to be considered healthy.
    /// Useful for applications that might temporarily have no resources.
    /// Default: false
    #[serde(
        rename = "allowEmpty",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_empty: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RetryStrategy {
    #[serde(default)]
    pub limit: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub backoff: Option<Backoff>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Backoff {
    /// e.g., "5s"
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub duration: String,

    /// e.g., 2
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub factor: Option<i64>,

    /// e.g., "3m"
    #[serde(
        rename = "maxDuration",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub max_duration: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManagedNamespaceMetadata {
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub labels: BTreeMap<String, String>,

    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub annotations: BTreeMap<String, String>,
}

/// ---------------------------------------------------------------------
/// Source options: Kustomize / Helm / Directory / Plugin
/// ---------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KustomizeOptions {
    #[serde(
        rename = "namePrefix",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub name_prefix: String,

    #[serde(
        rename = "nameSuffix",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub name_suffix: String,

    /// Kustomize images override (Argo's Kustomize uses plain strings)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<String>,

    #[serde(
        rename = "commonLabels",
        default,
        skip_serializing_if = "BTreeMap::is_empty"
    )]
    pub common_labels: BTreeMap<String, String>,

    #[serde(
        rename = "commonAnnotations",
        default,
        skip_serializing_if = "BTreeMap::is_empty"
    )]
    pub common_annotations: BTreeMap<String, String>,

    /// Optionally, path to kustomization file (rare)
    #[serde(
        rename = "kustomizePath",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub kustomize_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HelmOptions {
    /// Inline values.yaml string
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub values: String,

    /// File paths relative to repo root
    #[serde(rename = "valueFiles", default, skip_serializing_if = "Vec::is_empty")]
    pub value_files: Vec<String>,

    /// --set name=value
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<HelmParameter>,

    /// --set-file name=path
    #[serde(
        rename = "fileParameters",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub file_parameters: Vec<HelmFileParameter>,

    #[serde(
        rename = "releaseName",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub release_name: String,

    #[serde(
        rename = "passCredentials",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub pass_credentials: Option<bool>,

    #[serde(
        rename = "ignoreMissingValueFiles",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_missing_value_files: Option<bool>,

    #[serde(rename = "skipCrds", default, skip_serializing_if = "Option::is_none")]
    pub skip_crds: Option<bool>,

    /// `helm dependency update` before template
    #[serde(
        rename = "helmDependencyUpdate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub helm_dependency_update: Option<bool>,

    /// Additional Helm flags (template/install/upgrade)
    #[serde(
        rename = "parametersList",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub parameters_list: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HelmParameter {
    #[serde(default)]
    pub name: String,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub value: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub r#force_string: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HelmFileParameter {
    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DirectoryOptions {
    /// Recurse into subdirs
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recurse: Option<bool>,

    /// Glob of files to exclude
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub exclude: String,

    /// Glob of files to include
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub include: String,

    /// Jsonnet settings if the directory contains Jsonnet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsonnet: Option<JsonnetOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JsonnetOptions {
    /// `jsonnet -J` search paths
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub libs: Vec<String>,

    /// External variables (`--ext-str/--ext-code`)
    #[serde(rename = "extVars", default, skip_serializing_if = "Vec::is_empty")]
    pub ext_vars: Vec<JsonnetVar>,

    /// Top-level arguments (`--tla-str/--tla-code`)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tlas: Vec<JsonnetVar>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JsonnetVar {
    #[serde(default)]
    pub name: String,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub value: String,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PluginOptions {
    /// CMP name
    #[serde(default)]
    pub name: String,

    /// Environment variables passed to the plugin
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub env: Vec<EnvEntry>,

    /// Arbitrary string parameters (plugin-defined)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parameters: Vec<PluginParameter>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnvEntry {
    #[serde(default)]
    pub name: String,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PluginParameter {
    #[serde(default)]
    pub name: String,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub string: String,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub array: Vec<String>,

    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub map: BTreeMap<String, String>,
}

/// Optional cross-ref to a named source (advanced)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApplicationSourceRef {
    /// "Application"
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub kind: String,

    /// usually same namespace
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub namespace: String,

    #[serde(rename = "group", default, skip_serializing_if = "String::is_empty")]
    pub api_group: String,
}

/// ---------------------------------------------------------------------
/// spec.ignoreDifferences[*]
/// ---------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceIgnoreDifference {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub group: String,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub kind: String,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub namespace: String,

    /// e.g., ["/spec/template/spec/containers/0/imagePullPolicy"]
    #[serde(
        rename = "jsonPointers",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub json_pointers: Vec<String>,

    /// jq expressions
    #[serde(
        rename = "jqPathExpressions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub jq_path_expressions: Vec<String>,

    /// Ignore any change caused by these managers in managedFields
    #[serde(
        rename = "managedFieldsManagers",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub managed_fields_managers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InfoItem {
    #[serde(default)]
    pub name: String,

    #[serde(default)]
    pub value: String,
}

/// ---------------------------------------------------------------------
/// status  (trimmed to commonly used fields; extend as needed)
/// ---------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApplicationStatus {
    /// Sync status (Synced/OutOfSync)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync: Option<SyncStatus>,

    /// Health status (Healthy/Progressing/Degraded/Unknown)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub health: Option<HealthStatus>,

    /// Current operation (if any)
    #[serde(rename = "operationState", skip_serializing_if = "Option::is_none")]
    pub operation_state: Option<OperationState>,

    /// Last reconciliation / observation timestamps
    #[serde(
        rename = "reconciledAt",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub reconciled_at: String,

    #[serde(
        rename = "observedAt",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub observed_at: String,

    /// Condition list
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conditions: Vec<ApplicationCondition>,

    /// Recent sync history
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub history: Vec<RevisionHistory>,

    /// Aggregates basic counts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<ApplicationSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SyncStatus {
    #[serde(default)]
    pub status: String, // "Synced" | "OutOfSync" | "Unknown"

    #[serde(rename = "comparedTo", skip_serializing_if = "Option::is_none")]
    pub compared_to: Option<ComparedTo>,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub revision: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComparedTo {
    #[serde(default)]
    pub destination: ApplicationDestination,

    #[serde(default)]
    pub source: ApplicationSource,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HealthStatus {
    #[serde(default)]
    pub status: String, // "Healthy" | "Progressing" | "Degraded" | "Suspended" | "Missing" | "Unknown"

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OperationState {
    #[serde(default)]
    pub phase: String, // "Succeeded" | "Running" | "Failed" | "Error"

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,

    #[serde(
        rename = "startedAt",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub started_at: String,

    #[serde(
        rename = "finishedAt",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub finished_at: String,

    #[serde(rename = "syncResult", skip_serializing_if = "Option::is_none")]
    pub sync_result: Option<SyncOperationResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SyncOperationResult {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub resources: Vec<ResourceResult>,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub revision: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ApplicationSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceResult {
    #[serde(default)]
    pub group: String,

    #[serde(default)]
    pub kind: String,

    #[serde(default)]
    pub namespace: String,

    #[serde(default)]
    pub name: String,

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub status: String, // "Synced", etc.

    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub message: String,

    #[serde(rename = "hookType", default, skip_serializing_if = "String::is_empty")]
    pub hook_type: String,

    #[serde(
        rename = "hookPhase",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub hook_phase: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApplicationCondition {
    #[serde(rename = "type", default)]
    pub r#type: String,

    #[serde(default)]
    pub message: String,

    #[serde(
        rename = "lastTransitionTime",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub last_transition_time: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RevisionHistory {
    #[serde(default)]
    pub revision: String,

    #[serde(
        rename = "deployedAt",
        default,
        skip_serializing_if = "String::is_empty"
    )]
    pub deployed_at: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ApplicationSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApplicationSummary {
    #[serde(
        rename = "externalURLs",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub external_urls: Vec<String>,

    #[serde(rename = "images", default, skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<String>,
}
