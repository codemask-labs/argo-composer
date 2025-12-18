#![allow(dead_code)]

use yaml::prelude::*;

/// Kustomization image configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KustomizationImage {}

/// Kustomization resource for Kustomize-based deployments
/// API Version: kustomize.config.k8s.io/v1beta1
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kustomization {
    /// API version of the Kustomization resource
    pub api_version: String,

    /// Resource kind, always "Kustomization"
    pub kind: String,

    /// List of resource file paths to include
    pub resources: Vec<String>,

    /// List of images to transform
    pub images: Option<Vec<KustomizationImage>>,
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
            images: None,
        }
    }
}
