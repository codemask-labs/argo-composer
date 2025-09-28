use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

use crate::resources::ApplicationDestination;

/// argoproj.io/v1alpha1 AppProject (CRD)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppProject {
    #[serde(rename = "apiVersion")]
    pub api_version: String, // "argoproj.io/v1alpha1"

    #[serde(rename = "kind")]
    pub kind: String, // "AppProject"

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ObjectMeta>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AppProjectStatus>,

    pub spec: AppProjectSpec,
}

impl Default for AppProject {
    fn default() -> Self {
        AppProject {
            api_version: "argoproj.io/v1alpha1".to_string(),
            kind: "AppProject".to_string(),
            metadata: None,
            status: None,
            spec: AppProjectSpec::default(),
        }
    }
}

/// Spec section
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppProjectSpec {
    #[serde(rename = "sourceRepos", default)]
    pub source_repos: Vec<String>,

    #[serde(default)]
    pub destinations: Vec<ApplicationDestination>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roles: Vec<ProjectRole>,

    #[serde(
        rename = "clusterResourceWhitelist",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub cluster_resource_whitelist: Vec<GroupKind>,

    #[serde(
        rename = "namespaceResourceBlacklist",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub namespace_resource_blacklist: Vec<GroupKind>,

    #[serde(rename = "orphanedResources", skip_serializing_if = "Option::is_none")]
    pub orphaned_resources: Option<OrphanedResourcesMonitorSettings>,

    #[serde(rename = "syncWindows", skip_serializing_if = "Vec::is_empty")]
    pub sync_windows: Vec<SyncWindow>,

    #[serde(rename = "namespaceResourceWhitelist", default)]
    pub namespace_resource_whitelist: Vec<GroupKind>,

    #[serde(rename = "signatureKeys", skip_serializing_if = "Vec::is_empty")]
    pub signature_keys: Vec<SignatureKey>,

    #[serde(
        rename = "clusterResourceBlacklist",
        skip_serializing_if = "Vec::is_empty"
    )]
    pub cluster_resource_blacklist: Vec<GroupKind>,
}

/// Status section
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppProjectStatus {
    #[serde(rename = "jwtTokensByRole", default)]
    pub jwt_tokens_by_role: BTreeMap<String, JwtTokens>,
}

/// GroupKind (cluster/namespace allow/deny lists)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct GroupKind {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,

    pub kind: String,
}

/// ProjectRole with policies and JWT tokens
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ProjectRole {
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(default)]
    pub policies: Vec<String>,

    #[serde(rename = "jwtTokens", default)]
    pub jwt_tokens: Vec<JwtToken>,

    #[serde(default)]
    pub groups: Vec<String>,
}

/// OrphanedResources settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrphanedResourcesMonitorSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warn: Option<bool>,

    #[serde(default)]
    pub ignore: Vec<OrphanedResourceKey>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct OrphanedResourceKey {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// SyncWindow (allow/deny by cron)
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SyncWindow {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<String>,

    #[serde(default)]
    pub applications: Vec<String>,

    #[serde(default)]
    pub namespaces: Vec<String>,

    #[serde(default)]
    pub clusters: Vec<String>,

    #[serde(rename = "manualSync", skip_serializing_if = "Option::is_none")]
    pub manual_sync: Option<bool>,
}

/// SignatureKey (for commit signature verification)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct SignatureKey {
    #[serde(rename = "keyID")]
    pub key_id: String,
}

/// JWTToken
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq)]
pub struct JwtToken {
    #[serde(rename = "iat")]
    pub issued_at: i64,

    #[serde(rename = "exp", skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

/// Wrapper for a list of JWT tokens
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct JwtTokens {
    #[serde(default)]
    pub items: Vec<JwtToken>,
}
