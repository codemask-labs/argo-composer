use std::collections::BTreeMap;
use yaml::prelude::*;

/// Standard Kubernetes object metadata
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ObjectMeta {
    /// Name of the resource
    pub name: Option<String>,

    /// Namespace of the resource
    pub namespace: Option<String>,

    /// Labels attached to the resource
    pub labels: Option<BTreeMap<String, String>>,

    /// Annotations attached to the resource
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
