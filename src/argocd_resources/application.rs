use std::collections::BTreeMap;
use yaml::prelude::*;

/// Metadata for ArgoCD Application resources
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApplicationMetadata {
    /// Name of the Application
    pub name: Option<String>,

    /// Namespace of the Application (typically 'argocd')
    pub namespace: Option<String>,

    /// Labels attached to the Application
    pub labels: Option<BTreeMap<String, String>>,

    /// Annotations attached to the Application
    pub annotations: Option<BTreeMap<String, String>>,

    /// Finalizers list
    pub finalizers: Option<Vec<String>>,
}

/// Destination cluster/namespace for ArgoCD applications
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApplicationDestination {
    /// Server URL for the target Kubernetes cluster
    /// Either set 'server' OR 'name' (named cluster reference)
    pub server: Option<String>,

    /// Named cluster reference (alternative to server URL)
    pub name: Option<String>,

    /// Target namespace in the cluster
    pub namespace: Option<String>,
}

/// ArgoCD Application resource
/// API Version: argoproj.io/v1alpha1
///
/// Application CRD represents a deployed application instance in an environment.
/// It describes the source (Git repository or Helm chart) and destination (Kubernetes cluster and namespace)
/// for deploying Kubernetes manifests.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Application {
    /// API version, always "argoproj.io/v1alpha1"
    #[yaml(rename = "apiVersion")]
    pub api_version: String,

    /// Resource kind, always "Application"
    pub kind: String,

    /// Standard Kubernetes object metadata
    pub metadata: Option<ApplicationMetadata>,

    /// Desired state specification
    pub spec: ApplicationSpec,
}

impl Default for Application {
    fn default() -> Self {
        Application {
            api_version: "argoproj.io/v1alpha1".to_string(),
            kind: "Application".to_string(),
            metadata: None,
            spec: ApplicationSpec::default(),
        }
    }
}

/// ApplicationSpec defines the desired state of the Application
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApplicationSpec {
    /// Project name this application belongs to
    pub project: Option<String>,

    /// Single source for the application (use either source or sources, not both)
    pub source: Option<ApplicationSource>,

    /// Multiple sources for the application (use either source or sources, not both)
    pub sources: Vec<ApplicationSource>,

    /// Destination cluster and namespace
    pub destination: Option<ApplicationDestination>,

    /// Sync policy for automated sync and retry behavior
    #[yaml(rename = "syncPolicy")]
    pub sync_policy: Option<SyncPolicy>,

    /// List of resources to ignore differences
    #[yaml(rename = "ignoreDifferences")]
    pub ignore_differences: Vec<ResourceIgnoreDifference>,

    /// Additional information displayed in UI
    pub info: Vec<InfoItem>,

    /// Reference to another Application's source
    #[yaml(rename = "sourceRef")]
    pub source_ref: Option<ApplicationSourceRef>,
}

/// ApplicationSource contains information about the source of application manifests
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApplicationSource {
    /// URL to the repository (Git or Helm)
    #[yaml(rename = "repoUrl")]
    pub repo_url: String,

    /// Directory path within the Git repository
    pub path: String,

    /// Revision to sync (branch, tag, commit SHA, or chart version)
    #[yaml(rename = "targetRevision")]
    pub target_revision: String,

    /// Helm chart name (for Helm repositories)
    pub chart: String,

    /// Kustomize-specific options
    pub kustomize: Option<KustomizeOptions>,

    /// Helm-specific options
    pub helm: Option<HelmOptions>,

    /// Directory-specific options
    pub directory: Option<DirectoryOptions>,

    /// Plugin-specific options
    pub plugin: Option<PluginOptions>,

    /// Symbolic reference name for multi-source applications
    #[yaml(rename = "ref")]
    pub ref_name: String,
}

/// SyncPolicy controls when and how a sync is performed
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SyncPolicy {
    /// Automated sync configuration
    pub automated: Option<SyncPolicyAutomated>,

    /// Additional sync options
    #[yaml(rename = "syncOptions")]
    pub sync_options: Vec<String>,

    /// Retry strategy for failed syncs
    pub retry: Option<RetryStrategy>,

    /// Metadata for managed namespace
    #[yaml(rename = "managedNamespaceMetadata")]
    pub managed_namespace_metadata: Option<ManagedNamespaceMetadata>,
}

/// Automated sync policy configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SyncPolicyAutomated {
    /// Automatically prune resources when they are no longer defined in Git
    pub prune: Option<bool>,

    /// Automatically sync when cluster state deviates from Git
    pub self_heal: Option<bool>,

    /// Allow deletion of resources that would become orphaned
    pub allow_empty: Option<bool>,
}

/// Retry strategy for failed syncs
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RetryStrategy {
    /// Maximum number of retry attempts
    pub limit: Option<i64>,

    /// Backoff configuration
    pub backoff: Option<Backoff>,
}

/// Backoff configuration for retries
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Backoff {
    /// Initial backoff duration
    pub duration: Option<String>,

    /// Backoff multiplier factor
    pub factor: Option<i64>,

    /// Maximum backoff duration
    #[yaml(rename = "maxDuration")]
    pub max_duration: Option<String>,
}

/// Metadata to apply to managed namespace
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ManagedNamespaceMetadata {
    /// Labels to apply
    pub labels: Option<BTreeMap<String, String>>,

    /// Annotations to apply
    pub annotations: Option<BTreeMap<String, String>>,
}

/// Kustomize-specific options
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct KustomizeOptions {
    /// Version of Kustomize to use
    pub version: Option<String>,

    /// Name prefix for all resources
    #[yaml(rename = "namePrefix")]
    pub name_prefix: Option<String>,

    /// Name suffix for all resources
    #[yaml(rename = "nameSuffix")]
    pub name_suffix: Option<String>,

    /// Images to override
    pub images: Vec<String>,

    /// Common labels to add to all resources
    #[yaml(rename = "commonLabels")]
    pub common_labels: Option<BTreeMap<String, String>>,

    /// Common annotations to add to all resources
    #[yaml(rename = "commonAnnotations")]
    pub common_annotations: Option<BTreeMap<String, String>>,

    /// Namespace to set for all resources
    pub namespace: Option<String>,

    /// Force common labels
    #[yaml(rename = "forceCommonLabels")]
    pub force_common_labels: Option<bool>,

    /// Force common annotations
    #[yaml(rename = "forceCommonAnnotations")]
    pub force_common_annotations: Option<bool>,
}

/// Helm-specific options
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HelmOptions {
    /// Helm values files paths
    #[yaml(rename = "valueFiles")]
    pub value_files: Vec<String>,

    /// Values as YAML string
    pub values: Option<String>,

    /// Release name override
    #[yaml(rename = "releaseName")]
    pub release_name: Option<String>,

    /// Individual parameter values
    pub parameters: Vec<HelmParameter>,

    /// File-based parameter values
    #[yaml(rename = "fileParameters")]
    pub file_parameters: Vec<HelmFileParameter>,

    /// Helm version to use
    pub version: Option<String>,

    /// Pass credentials to all domains
    #[yaml(rename = "passCredentials")]
    pub pass_credentials: Option<bool>,

    /// Ignore missing value files
    #[yaml(rename = "ignoreMissingValueFiles")]
    pub ignore_missing_value_files: Option<bool>,

    /// Skip CRD installation
    #[yaml(rename = "skipCrds")]
    pub skip_crds: Option<bool>,
}

/// Helm parameter
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HelmParameter {
    /// Parameter name
    pub name: String,

    /// Parameter value
    pub value: String,

    /// Force string type
    pub force_string: Option<bool>,
}

/// Helm file parameter
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HelmFileParameter {
    /// Parameter name
    pub name: String,

    /// File path
    pub path: String,
}

/// Directory-specific options
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DirectoryOptions {
    /// Recurse into subdirectories
    pub recurse: Option<bool>,

    /// Jsonnet-specific options
    pub jsonnet: Option<JsonnetOptions>,

    /// File inclusion patterns
    pub include: Option<String>,

    /// File exclusion patterns
    pub exclude: Option<String>,
}

/// Jsonnet-specific options
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JsonnetOptions {
    /// External variables
    pub ext_vars: Vec<JsonnetVar>,

    /// Top-level arguments
    pub tlas: Vec<JsonnetVar>,

    /// Library paths
    pub libs: Vec<String>,
}

/// Jsonnet variable
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JsonnetVar {
    /// Variable name
    pub name: String,

    /// Variable value
    pub value: String,

    /// Read value from file
    pub code: Option<bool>,
}

/// Plugin-specific options (Config Management Plugin)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PluginOptions {
    /// Plugin name
    pub name: String,

    /// Environment variables
    pub env: Vec<EnvEntry>,

    /// Plugin parameters
    pub parameters: Vec<PluginParameter>,
}

/// Environment variable entry
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EnvEntry {
    /// Variable name
    pub name: String,

    /// Variable value
    pub value: String,
}

/// Plugin parameter
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PluginParameter {
    /// Parameter name
    pub name: Option<String>,

    /// Parameter value (string, number, array, or map)
    pub string_value: Option<String>,

    /// Array value
    pub array: Vec<String>,

    /// Map value
    pub map: Option<BTreeMap<String, String>>,
}

/// Reference to another Application's source
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApplicationSourceRef {
    /// Target revision
    pub target_revision: String,

    /// Chart name
    pub chart: Option<String>,
}

/// Resource to ignore differences during sync
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResourceIgnoreDifference {
    /// Resource group
    pub group: Option<String>,

    /// Resource kind
    pub kind: String,

    /// Resource name pattern
    pub name: Option<String>,

    /// Namespace pattern
    pub namespace: Option<String>,

    /// JSON paths to ignore
    pub json_pointers: Vec<String>,

    /// JQ path expressions to ignore
    pub jq_path_expressions: Vec<String>,

    /// Managed fields managers to ignore
    pub managed_fields_managers: Vec<String>,
}

/// Additional information item for UI display
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InfoItem {
    /// Information name/key
    pub name: String,

    /// Information value
    pub value: String,
}

/// ApplicationStatus represents the current state of the application
#[derive(Debug, Clone, Default)]
pub struct ApplicationStatus {
    /// Sync status
    pub sync: Option<SyncStatus>,

    /// Health status
    pub health: Option<HealthStatus>,

    /// Operation state
    pub operation_state: Option<OperationState>,

    /// Conditions
    pub conditions: Vec<ApplicationCondition>,

    /// Revision history
    pub history: Vec<RevisionHistory>,

    /// Summary information
    pub summary: Option<ApplicationSummary>,
}

/// Sync status of the application
#[derive(Debug, Clone, Default)]
pub struct SyncStatus {
    /// Sync status: Synced, OutOfSync, Unknown
    pub status: String,

    /// Revision of the sync
    pub revision: Option<String>,

    /// Source compared to
    pub compared_to: Option<ComparedTo>,
}

/// Source that was compared during sync
#[derive(Debug, Clone, Default)]
pub struct ComparedTo {
    /// Source specification
    pub source: ApplicationSource,

    /// Destination
    pub destination: ApplicationDestination,
}

/// Health status of the application
#[derive(Debug, Clone, Default)]
pub struct HealthStatus {
    /// Health status: Healthy, Progressing, Degraded, Suspended, Missing, Unknown
    pub status: String,

    /// Health status message
    pub message: Option<String>,
}

/// State of ongoing operation
#[derive(Debug, Clone, Default)]
pub struct OperationState {
    /// Operation being performed
    pub operation: Option<String>,

    /// Phase: Running, Error, Failed, Succeeded
    pub phase: Option<String>,

    /// Human-readable message
    pub message: Option<String>,

    /// Start timestamp
    pub started_at: Option<String>,

    /// Finish timestamp
    pub finished_at: Option<String>,

    /// Retry count
    pub retry_count: Option<i64>,

    /// Sync result
    pub sync_result: Option<SyncOperationResult>,
}

/// Result of a sync operation
#[derive(Debug, Clone, Default)]
pub struct SyncOperationResult {
    /// Resources synced
    pub resources: Vec<ResourceResult>,

    /// Revision synced to
    pub revision: Option<String>,

    /// Source used
    pub source: Option<ApplicationSource>,
}

/// Result of syncing a single resource
#[derive(Debug, Clone, Default)]
pub struct ResourceResult {
    /// Resource group
    pub group: Option<String>,

    /// Resource version
    pub version: String,

    /// Resource kind
    pub kind: String,

    /// Resource namespace
    pub namespace: Option<String>,

    /// Resource name
    pub name: String,

    /// Sync status
    pub status: Option<String>,

    /// Result message
    pub message: Option<String>,

    /// Hook type
    pub hook_type: Option<String>,

    /// Hook phase
    pub hook_phase: Option<String>,

    /// Sync phase
    pub sync_phase: Option<String>,
}

/// Condition of an application
#[derive(Debug, Clone, Default)]
pub struct ApplicationCondition {
    /// Condition type
    pub condition_type: String,

    /// Condition message
    pub message: String,

    /// Last transition time
    pub last_transition_time: Option<String>,
}

/// Historical information about application syncs
#[derive(Debug, Clone, Default)]
pub struct RevisionHistory {
    /// Revision
    pub revision: String,

    /// Deployed at timestamp
    pub deployed_at: String,

    /// Deployment ID
    pub id: Option<i64>,

    /// Source used
    pub source: Option<ApplicationSource>,
}

/// Summary of application state
#[derive(Debug, Clone, Default)]
pub struct ApplicationSummary {
    /// External URLs
    pub external_urls: Vec<String>,

    /// Images deployed
    pub images: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use yaml::Yaml;

    #[test]
    fn test_application_default() {
        let app = Application::default();

        assert_eq!(app.api_version, "argoproj.io/v1alpha1");
        assert_eq!(app.kind, "Application");
        assert!(app.metadata.is_none());

        // Verify that optional None fields are not serialized (including their comments)
        let yaml_output = Yaml::serialize_to_string(app).expect("Failed to serialize");
        println!("Minimal Application YAML:\n{}", yaml_output);

        // metadata should not appear (it's None)
        assert!(!yaml_output.contains("metadata"));
        // But required fields should appear
        assert!(yaml_output.contains("apiVersion"));
        assert!(yaml_output.contains("spec"));
    }

    #[test]
    fn test_application_serialization() {
        let mut app = Application::default();

        // Set metadata
        app.metadata = Some(ApplicationMetadata {
            name: Some("my-app".to_string()),
            namespace: Some("argocd".to_string()),
            labels: None,
            annotations: None,
            finalizers: None,
        });

        // Set spec
        app.spec.project = Some("default".to_string());
        app.spec.source = Some(ApplicationSource {
            repo_url: "https://github.com/argoproj/argocd-example-apps".to_string(),
            path: "guestbook".to_string(),
            target_revision: "HEAD".to_string(),
            ..Default::default()
        });
        app.spec.destination = Some(ApplicationDestination {
            server: Some("https://kubernetes.default.svc".to_string()),
            namespace: Some("default".to_string()),
            name: None,
        });

        let yaml_output = Yaml::serialize_to_string(app).expect("Failed to serialize");

        println!("Serialized YAML:\n{}", yaml_output);

        // Verify key fields are present in YAML (with camelCase field names)
        assert!(yaml_output.contains("apiVersion"));
        assert!(yaml_output.contains("kind"));
        assert!(yaml_output.contains("Application"));
    }

    #[test]
    fn test_application_with_sync_policy() {
        let mut app = Application::default();

        app.spec.sync_policy = Some(SyncPolicy {
            automated: Some(SyncPolicyAutomated {
                prune: Some(true),
                self_heal: Some(true),
                allow_empty: Some(false),
            }),
            sync_options: vec![
                "CreateNamespace=true".to_string(),
                "PruneLast=true".to_string(),
            ],
            retry: Some(RetryStrategy {
                limit: Some(5),
                backoff: Some(Backoff {
                    duration: Some("5s".to_string()),
                    factor: Some(2),
                    max_duration: Some("3m".to_string()),
                }),
            }),
            managed_namespace_metadata: None,
        });

        let yaml_output = Yaml::serialize_to_string(app).expect("Failed to serialize");

        assert!(yaml_output.contains("syncPolicy"));
        assert!(yaml_output.contains("automated"));
    }

    #[test]
    fn test_application_with_helm_source() {
        let source = ApplicationSource {
            repo_url: "https://charts.example.com".to_string(),
            chart: "nginx".to_string(),
            target_revision: "1.2.3".to_string(),
            helm: Some(HelmOptions {
                value_files: vec!["values-prod.yaml".to_string()],
                values: Some("replicas: 3\nimage:\n  tag: v1.2.3".to_string()),
                parameters: vec![HelmParameter {
                    name: "service.type".to_string(),
                    value: "LoadBalancer".to_string(),
                    force_string: Some(false),
                }],
                release_name: Some("my-nginx".to_string()),
                ..Default::default()
            }),
            ..Default::default()
        };

        let mut app = Application::default();
        app.spec.source = Some(source);

        let yaml_output = Yaml::serialize_to_string(app).expect("Failed to serialize");

        assert!(yaml_output.contains("helm"));
        assert!(yaml_output.contains("nginx"));
    }

    #[test]
    fn test_application_with_kustomize_source() {
        let source = ApplicationSource {
            repo_url: "https://github.com/myorg/myapp".to_string(),
            path: "overlays/production".to_string(),
            target_revision: "main".to_string(),
            kustomize: Some(KustomizeOptions {
                images: vec!["myapp=myapp:v1.2.3".to_string()],
                name_prefix: Some("prod-".to_string()),
                namespace: Some("production".to_string()),
                common_labels: Some(
                    [("env".to_string(), "production".to_string())]
                        .iter()
                        .cloned()
                        .collect(),
                ),
                ..Default::default()
            }),
            ..Default::default()
        };

        let mut app = Application::default();
        app.spec.source = Some(source);

        let yaml_output = Yaml::serialize_to_string(app).expect("Failed to serialize");

        assert!(yaml_output.contains("kustomize"));
    }

    #[test]
    fn test_application_round_trip() {
        let mut original = Application::default();
        original.metadata = Some(ApplicationMetadata {
            name: Some("test-app".to_string()),
            namespace: Some("argocd".to_string()),
            labels: None,
            annotations: None,
            finalizers: None,
        });
        original.spec.project = Some("default".to_string());
        original.spec.source = Some(ApplicationSource {
            repo_url: "https://github.com/test/repo".to_string(),
            path: "manifests".to_string(),
            target_revision: "HEAD".to_string(),
            ..Default::default()
        });

        let yaml_string = Yaml::serialize_to_string(original.clone()).expect("Failed to serialize");
        let deserialized: Application =
            Yaml::from_string(&yaml_string).expect("Failed to deserialize");

        assert_eq!(deserialized.api_version, "argoproj.io/v1alpha1");
        assert_eq!(deserialized.kind, "Application");
        assert!(deserialized.metadata.is_some());
    }
}
