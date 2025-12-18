use std::collections::BTreeMap;
use yaml::prelude::*;

use crate::argocd_resources::{ApplicationDestination, ObjectMeta};

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
    pub metadata: Option<ObjectMeta>,

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
    pub destinations: Vec<ApplicationDestination>,

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
