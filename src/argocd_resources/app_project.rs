use std::collections::BTreeMap;
use yaml::prelude::*;

/// Metadata for ArgoCD AppProject resources
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppProjectMetadata {
    /// Name of the AppProject
    pub name: Option<String>,

    /// Namespace of the AppProject (typically 'argocd')
    pub namespace: Option<String>,

    /// Labels attached to the AppProject
    pub labels: Option<BTreeMap<String, String>>,

    /// Annotations attached to the AppProject
    pub annotations: Option<BTreeMap<String, String>>,

    /// Finalizers list
    pub finalizers: Option<Vec<String>>,
}

/// Destination cluster/namespace for ArgoCD projects
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectDestination {
    /// Server URL for the target Kubernetes cluster
    /// Either set 'server' OR 'name' (named cluster reference)
    pub server: Option<String>,

    /// Named cluster reference (alternative to server URL)
    pub name: Option<String>,

    /// Target namespace in the cluster
    pub namespace: Option<String>,
}

/// ArgoCD AppProject resource
/// API Version: argoproj.io/v1alpha1
///
/// AppProject provides a logical grouping of applications with governance and resource constraints.
/// Projects restrict where applications can deploy and what resources they can use.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppProject {
    /// API version, always "argoproj.io/v1alpha1"
    #[yaml(rename = "apiVersion")]
    pub api_version: String,

    /// Resource kind, always "AppProject"
    pub kind: String,

    /// Standard Kubernetes object metadata
    pub metadata: Option<AppProjectMetadata>,

    /// Desired state specification
    pub spec: AppProjectSpec,
}

impl Default for AppProject {
    fn default() -> Self {
        AppProject {
            api_version: "argoproj.io/v1alpha1".to_string(),
            kind: "AppProject".to_string(),
            metadata: None,
            spec: AppProjectSpec::default(),
        }
    }
}

/// AppProject specification defining project constraints and policies
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppProjectSpec {
    /// List of allowed source repositories (Git URLs or Helm repos)
    #[yaml(rename = "sourceRepos")]
    pub source_repos: Vec<String>,

    /// List of allowed destination clusters and namespaces
    pub destinations: Vec<ProjectDestination>,

    /// Human-readable description of the project
    pub description: Option<String>,

    /// RBAC roles for project access control
    pub roles: Vec<ProjectRole>,

    /// Whitelist of cluster-scoped resources (if empty, all are denied)
    #[yaml(rename = "clusterResourceWhitelist")]
    pub cluster_resource_whitelist: Vec<GroupKind>,

    /// Blacklist of namespace-scoped resources
    #[yaml(rename = "namespaceResourceBlacklist")]
    pub namespace_resource_blacklist: Vec<GroupKind>,

    /// Settings for monitoring orphaned resources
    #[yaml(rename = "orphanedResources")]
    pub orphaned_resources: Option<OrphanedResourcesMonitorSettings>,

    /// Time windows when syncs are allowed or denied
    #[yaml(rename = "syncWindows")]
    pub sync_windows: Vec<SyncWindow>,

    /// Whitelist of namespace-scoped resources (if empty, all are allowed)
    #[yaml(rename = "namespaceResourceWhitelist")]
    pub namespace_resource_whitelist: Vec<GroupKind>,

    /// List of PGP key IDs for commit signature verification
    #[yaml(rename = "signatureKeys")]
    pub signature_keys: Vec<SignatureKey>,

    /// Blacklist of cluster-scoped resources
    #[yaml(rename = "clusterResourceBlacklist")]
    pub cluster_resource_blacklist: Vec<GroupKind>,
}

/// Current status of an AppProject
#[derive(Debug, Clone, Default)]
pub struct AppProjectStatus {
    /// JWT tokens indexed by role name
    pub jwt_tokens_by_role: Option<BTreeMap<String, JwtTokens>>,
}

/// Kubernetes resource Group and Kind
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct GroupKind {
    /// API group (empty string for core group)
    pub group: Option<String>,

    /// Resource kind
    pub kind: String,
}

/// Project role with associated policies and JWT tokens
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectRole {
    /// Role name
    pub name: String,

    /// Human-readable description
    pub description: Option<String>,

    /// List of Casbin policy rules
    pub policies: Vec<String>,

    /// JWT tokens issued for this role
    #[yaml(rename = "jwtTokens")]
    pub jwt_tokens: Vec<JwtToken>,

    /// OIDC group claims that map to this role
    pub groups: Vec<String>,
}

/// Settings for monitoring orphaned Kubernetes resources
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrphanedResourcesMonitorSettings {
    /// If true, display warnings for orphaned resources
    pub warn: Option<bool>,

    /// List of resources to ignore in orphan detection
    pub ignore: Vec<OrphanedResourceKey>,
}

/// Identifier for an orphaned resource to ignore
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct OrphanedResourceKey {
    /// API group
    pub group: Option<String>,

    /// Resource kind
    pub kind: Option<String>,

    /// Resource name
    pub name: Option<String>,
}

/// Time window for allowing or denying application syncs
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SyncWindow {
    /// Window type: "allow" or "deny"
    pub kind: Option<String>,

    /// Cron schedule defining when the window is active
    pub schedule: Option<String>,

    /// Duration of the sync window
    pub duration: Option<String>,

    /// Applications this window applies to (empty means all)
    pub applications: Vec<String>,

    /// Namespaces this window applies to (empty means all)
    pub namespaces: Vec<String>,

    /// Clusters this window applies to (empty means all)
    pub clusters: Vec<String>,

    /// If true, only manual syncs are allowed during this window
    #[yaml(rename = "manualSync")]
    pub manual_sync: Option<bool>,
}

/// PGP key identifier for commit signature verification
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct SignatureKey {
    /// PGP key ID
    #[yaml(rename = "keyID")]
    pub key_id: String,
}

/// JWT token for role-based access
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct JwtToken {
    /// Token issued-at timestamp (Unix time)
    #[yaml(rename = "iat")]
    pub issued_at: i64,

    /// Token expiration timestamp (Unix time)
    #[yaml(rename = "exp")]
    pub expires_at: Option<i64>,

    /// Token identifier
    pub id: Option<String>,
}

/// Collection of JWT tokens
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JwtTokens {
    /// List of JWT tokens
    pub items: Vec<JwtToken>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::argocd_resources::GroupKind;
    use yaml::Yaml;

    #[test]
    fn test_app_project_default() {
        let project = AppProject::default();

        assert_eq!(project.api_version, "argoproj.io/v1alpha1");
        assert_eq!(project.kind, "AppProject");
        assert!(project.metadata.is_none());
    }

    #[test]
    fn test_app_project_serialization() {
        let mut project = AppProject::default();

        project.metadata = Some(AppProjectMetadata {
            name: Some("my-project".to_string()),
            namespace: Some("argocd".to_string()),
            labels: None,
            annotations: None,
            finalizers: None,
        });

        project.spec.description = Some("Production project".to_string());
        project.spec.source_repos = vec![
            "https://github.com/myorg/*".to_string(),
            "https://charts.example.com".to_string(),
        ];
        project.spec.destinations = vec![ProjectDestination {
            server: Some("https://kubernetes.default.svc".to_string()),
            namespace: Some("production".to_string()),
            name: None,
        }];

        let yaml_output = Yaml::serialize_to_string(project).expect("Failed to serialize");

        println!("AppProject YAML:\n{}", yaml_output);

        assert!(yaml_output.contains("apiVersion"));
        assert!(yaml_output.contains("AppProject"));
        assert!(yaml_output.contains("sourceRepos"));
    }

    #[test]
    fn test_app_project_with_roles() {
        let mut project = AppProject::default();

        project.spec.roles = vec![ProjectRole {
            name: "admin".to_string(),
            description: Some("Admin role".to_string()),
            policies: vec![
                "p, proj:my-project:admin, applications, *, my-project/*, allow".to_string(),
            ],
            groups: vec!["my-org:admins".to_string()],
            jwt_tokens: vec![],
        }];

        let yaml_output = Yaml::serialize_to_string(project).expect("Failed to serialize");

        assert!(yaml_output.contains("roles"));
        assert!(yaml_output.contains("admin"));
    }

    #[test]
    fn test_app_project_with_resource_restrictions() {
        let mut project = AppProject::default();

        project.spec.cluster_resource_whitelist = vec![
            GroupKind {
                group: Some("".to_string()),
                kind: "Namespace".to_string(),
            },
            GroupKind {
                group: Some("rbac.authorization.k8s.io".to_string()),
                kind: "ClusterRole".to_string(),
            },
        ];

        project.spec.namespace_resource_blacklist = vec![GroupKind {
            group: Some("".to_string()),
            kind: "ResourceQuota".to_string(),
        }];

        let yaml_output = Yaml::serialize_to_string(project).expect("Failed to serialize");

        assert!(yaml_output.contains("clusterResourceWhitelist"));
        assert!(yaml_output.contains("namespaceResourceBlacklist"));
    }

    #[test]
    fn test_app_project_with_sync_windows() {
        let mut project = AppProject::default();

        project.spec.sync_windows = vec![
            SyncWindow {
                kind: Some("allow".to_string()),
                schedule: Some("0 9 * * 1-5".to_string()), // Weekdays 9 AM
                duration: Some("8h".to_string()),
                applications: vec![],
                namespaces: vec![],
                clusters: vec![],
                manual_sync: None,
            },
            SyncWindow {
                kind: Some("deny".to_string()),
                schedule: Some("0 0 * * 0,6".to_string()), // Weekends
                duration: Some("24h".to_string()),
                applications: vec![],
                namespaces: vec![],
                clusters: vec![],
                manual_sync: Some(true),
            },
        ];

        let yaml_output = Yaml::serialize_to_string(project).expect("Failed to serialize");

        assert!(yaml_output.contains("syncWindows"));
    }

    #[test]
    fn test_app_project_round_trip() {
        let mut original = AppProject::default();
        original.metadata = Some(AppProjectMetadata {
            name: Some("test-project".to_string()),
            namespace: Some("argocd".to_string()),
            labels: None,
            annotations: None,
            finalizers: None,
        });
        original.spec.description = Some("Test project".to_string());
        original.spec.source_repos = vec!["*".to_string()];

        let yaml_string = Yaml::serialize_to_string(original.clone()).expect("Failed to serialize");
        let deserialized: AppProject =
            Yaml::from_string(&yaml_string).expect("Failed to deserialize");

        assert_eq!(deserialized.api_version, "argoproj.io/v1alpha1");
        assert_eq!(deserialized.kind, "AppProject");
        assert!(deserialized.metadata.is_some());
    }
}
