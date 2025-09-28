#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kustomization {
    #[serde(rename = "apiVersion")]
    pub api_version: String,

    #[serde(rename = "kind")]
    pub kind: String,

    /// Always serialize, even when empty
    #[serde(default)]
    pub resources: Vec<String>,
}

impl Kustomization {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Kustomization {
    fn default() -> Self {
        Kustomization {
            api_version: "kustomize.config.k8s.io/v1beta1".to_string(),
            kind: "Kustomization".to_string(),
            resources: Vec::new(),
        }
    }
}
