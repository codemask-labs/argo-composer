use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// ---------------------------------------------------------------------
/// Top-level: argoproj.io/v1alpha1, kind: Application
/// ---------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Application {
    #[serde(rename = "apiVersion")]
    pub api_version: String, // "argoproj.io/v1alpha1"

    #[serde(rename = "kind")]
    pub kind: String, // "Application"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<ApplicationStatus>,

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

/// ---------------------------------------------------------------------
/// spec
/// ---------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApplicationSpec {
    /// Name of the AppProject this application belongs to
    pub project: String,

    /// Single-source (legacy/convenience)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ApplicationSource>,

    /// Multi-source (preferred for modern setups)
    #[serde(default)]
    pub sources: Vec<ApplicationSource>,

    /// Where to deploy
    pub destination: ApplicationDestination,

    /// Sync behavior
    #[serde(rename = "syncPolicy", skip_serializing_if = "Option::is_none")]
    pub sync_policy: Option<SyncPolicy>,

    /// Limit for stored revisions in status.history
    #[serde(
        rename = "revisionHistoryLimit",
        skip_serializing_if = "Option::is_none"
    )]
    pub revision_history_limit: Option<i64>,

    /// Per-resource diff ignores (e.g., managedFields, paths)
    #[serde(rename = "ignoreDifferences", default)]
    pub ignore_differences: Vec<ResourceIgnoreDifference>,

    /// Arbitrary info pairs shown in UI
    #[serde(default)]
    pub info: Vec<InfoItem>,

    /// Optional sources reference (advanced; rarely used directly in YAML)
    #[serde(rename = "sourceRef", skip_serializing_if = "Option::is_none")]
    pub source_ref: Option<ApplicationSourceRef>,
}

/// ---------------------------------------------------------------------
/// spec.source / spec.sources[*]
/// ---------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApplicationSource {
    /// Git/Helm repository URL
    #[serde(rename = "repoURL", skip_serializing_if = "Option::is_none")]
    pub repo_url: Option<String>,

    /// Git path within repo (for directory/kustomize/jsonnet)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,

    /// Git revision/semver range/commit/tag/branch; Helm chart version if chart set
    #[serde(rename = "targetRevision", skip_serializing_if = "Option::is_none")]
    pub target_revision: Option<String>,

    /// Helm chart name (when sourcing from chart repo or Helm folder)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chart: Option<String>,

    /// Kustomize options (if using Kustomize)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kustomize: Option<KustomizeOptions>,

    /// Helm options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub helm: Option<HelmOptions>,

    /// Directory generator options (recurse/include/exclude/jsonnet)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub directory: Option<DirectoryOptions>,

    /// Config Management Plugin (CMP) options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plugin: Option<PluginOptions>,

    /// Reference key for multi-source (sourceRef + sources)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
}

/// Destination cluster/namespace
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApplicationDestination {
    /// Either set 'server' OR 'name' (named cluster)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
}

/// Sync policy (automated, retries, options, managed namespace metadata)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SyncPolicy {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automated: Option<SyncPolicyAutomated>,

    /// E.g., "CreateNamespace=true", "ApplyOutOfSyncOnly=true", ...
    #[serde(rename = "syncOptions", default)]
    pub sync_options: Vec<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub retry: Option<RetryStrategy>,

    /// Optionally manage metadata for the destination namespace
    #[serde(
        rename = "managedNamespaceMetadata",
        skip_serializing_if = "Option::is_none"
    )]
    pub managed_namespace_metadata: Option<ManagedNamespaceMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SyncPolicyAutomated {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prune: Option<bool>,

    #[serde(rename = "selfHeal", skip_serializing_if = "Option::is_none")]
    pub self_heal: Option<bool>,

    #[serde(rename = "allowEmpty", skip_serializing_if = "Option::is_none")]
    pub allow_empty: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RetryStrategy {
    pub limit: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backoff: Option<Backoff>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Backoff {
    /// e.g., "5s"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,
    /// e.g., 2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub factor: Option<i64>,
    /// e.g., "3m"
    #[serde(rename = "maxDuration", skip_serializing_if = "Option::is_none")]
    pub max_duration: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManagedNamespaceMetadata {
    #[serde(default)]
    pub labels: BTreeMap<String, String>,
    #[serde(default)]
    pub annotations: BTreeMap<String, String>,
}

/// ---------------------------------------------------------------------
/// Source options: Kustomize / Helm / Directory / Plugin
/// ---------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KustomizeOptions {
    #[serde(rename = "namePrefix", skip_serializing_if = "Option::is_none")]
    pub name_prefix: Option<String>,
    #[serde(rename = "nameSuffix", skip_serializing_if = "Option::is_none")]
    pub name_suffix: Option<String>,

    /// Kustomize images override (Argo’s Kustomize uses plain strings)
    #[serde(default)]
    pub images: Vec<String>,

    #[serde(rename = "commonLabels", skip_serializing_if = "Option::is_none")]
    pub common_labels: Option<BTreeMap<String, String>>,
    #[serde(rename = "commonAnnotations", skip_serializing_if = "Option::is_none")]
    pub common_annotations: Option<BTreeMap<String, String>>,

    /// Optionally, path to kustomization file (rare)
    #[serde(rename = "kustomizePath", skip_serializing_if = "Option::is_none")]
    pub kustomize_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HelmOptions {
    /// Inline values.yaml string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<String>,

    /// File paths relative to repo root
    #[serde(rename = "valueFiles", default)]
    pub value_files: Vec<String>,

    /// --set name=value
    #[serde(default)]
    pub parameters: Vec<HelmParameter>,

    /// --set-file name=path
    #[serde(rename = "fileParameters", default)]
    pub file_parameters: Vec<HelmFileParameter>,

    #[serde(rename = "releaseName", skip_serializing_if = "Option::is_none")]
    pub release_name: Option<String>,

    #[serde(rename = "passCredentials", skip_serializing_if = "Option::is_none")]
    pub pass_credentials: Option<bool>,

    #[serde(
        rename = "ignoreMissingValueFiles",
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_missing_value_files: Option<bool>,

    #[serde(rename = "skipCrds", skip_serializing_if = "Option::is_none")]
    pub skip_crds: Option<bool>,

    /// `helm dependency update` before template
    #[serde(
        rename = "helmDependencyUpdate",
        skip_serializing_if = "Option::is_none"
    )]
    pub helm_dependency_update: Option<bool>,

    /// Additional Helm flags (template/install/upgrade)
    #[serde(rename = "parametersList", skip_serializing_if = "Option::is_none")]
    pub parameters_list: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HelmParameter {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#force_string: Option<bool>, // corresponds to --set-string
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HelmFileParameter {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DirectoryOptions {
    /// Recurse into subdirs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurse: Option<bool>,

    /// Glob of files to exclude
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exclude: Option<String>,

    /// Glob of files to include
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<String>,

    /// Jsonnet settings if the directory contains Jsonnet
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jsonnet: Option<JsonnetOptions>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JsonnetOptions {
    /// `jsonnet -J` search paths
    #[serde(default)]
    pub libs: Vec<String>,

    /// External variables (`--ext-str/--ext-code`)
    #[serde(rename = "extVars", default)]
    pub ext_vars: Vec<JsonnetVar>,

    /// Top-level arguments (`--tla-str/--tla-code`)
    #[serde(default)]
    pub tlas: Vec<JsonnetVar>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JsonnetVar {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<bool>, // true => treat value as code
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PluginOptions {
    /// CMP name
    pub name: String,

    /// Environment variables passed to the plugin
    #[serde(default)]
    pub env: Vec<EnvEntry>,

    /// Arbitrary string parameters (plugin-defined)
    #[serde(default)]
    pub parameters: Vec<PluginParameter>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnvEntry {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PluginParameter {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub string: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub array: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map: Option<BTreeMap<String, String>>,
}

/// Optional cross-ref to a named source (advanced)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApplicationSourceRef {
    /// "Application"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// usually same namespace
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(rename = "group", skip_serializing_if = "Option::is_none")]
    pub api_group: Option<String>,
}

/// ---------------------------------------------------------------------
/// spec.ignoreDifferences[*]
/// ---------------------------------------------------------------------
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceIgnoreDifference {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,

    /// e.g., ["/spec/template/spec/containers/0/imagePullPolicy"]
    #[serde(rename = "jsonPointers", default)]
    pub json_pointers: Vec<String>,

    /// jq expressions
    #[serde(rename = "jqPathExpressions", default)]
    pub jq_path_expressions: Vec<String>,

    /// Ignore any change caused by these managers in managedFields
    #[serde(rename = "managedFieldsManagers", default)]
    pub managed_fields_managers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InfoItem {
    pub name: String,
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
    #[serde(rename = "reconciledAt", skip_serializing_if = "Option::is_none")]
    pub reconciled_at: Option<String>,
    #[serde(rename = "observedAt", skip_serializing_if = "Option::is_none")]
    pub observed_at: Option<String>,

    /// Condition list
    #[serde(default)]
    pub conditions: Vec<ApplicationCondition>,

    /// Recent sync history
    #[serde(default)]
    pub history: Vec<RevisionHistory>,

    /// Aggregates basic counts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<ApplicationSummary>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SyncStatus {
    pub status: String, // "Synced" | "OutOfSync" | "Unknown"
    #[serde(rename = "comparedTo", skip_serializing_if = "Option::is_none")]
    pub compared_to: Option<ComparedTo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ComparedTo {
    pub destination: ApplicationDestination,
    pub source: ApplicationSource,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HealthStatus {
    pub status: String, // "Healthy" | "Progressing" | "Degraded" | "Suspended" | "Missing" | "Unknown"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OperationState {
    pub phase: String, // "Succeeded" | "Running" | "Failed" | "Error"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "startedAt", skip_serializing_if = "Option::is_none")]
    pub started_at: Option<String>,
    #[serde(rename = "finishedAt", skip_serializing_if = "Option::is_none")]
    pub finished_at: Option<String>,
    #[serde(rename = "syncResult", skip_serializing_if = "Option::is_none")]
    pub sync_result: Option<SyncOperationResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SyncOperationResult {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<Vec<ResourceResult>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub revision: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ApplicationSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceResult {
    pub group: String,
    pub kind: String,
    pub namespace: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>, // "Synced", etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "hookType", skip_serializing_if = "Option::is_none")]
    pub hook_type: Option<String>,
    #[serde(rename = "hookPhase", skip_serializing_if = "Option::is_none")]
    pub hook_phase: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApplicationCondition {
    #[serde(rename = "type")]
    pub r#type: String,
    pub message: String,
    pub lastTransitionTime: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RevisionHistory {
    pub revision: String,
    #[serde(rename = "deployedAt", skip_serializing_if = "Option::is_none")]
    pub deployed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<ApplicationSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApplicationSummary {
    #[serde(rename = "externalURLs", default)]
    pub external_urls: Vec<String>,
    #[serde(rename = "images", default)]
    pub images: Vec<String>,
}
