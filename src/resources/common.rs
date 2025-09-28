use serde::{Deserialize, Serialize};

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
