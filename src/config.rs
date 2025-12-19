use std::path::PathBuf;

use yaml::{Deserialize, Serialize, Yaml};

#[derive(Debug)]
pub enum ConfigError {
    FailedToDeserialize,
    InvalidConfigurationKind,
}

// #[derive(Serialize, Deserialize, Debug)]
#[derive(Debug, Serialize, Deserialize)]
pub struct KuberentesConfig {
    // #[serde(default)]
    pub version: String,
}

impl Default for KuberentesConfig {
    fn default() -> Self {
        Self {
            version: String::from("latest"),
        }
    }
}

// #[derive(Serialize, Deserialize, Debug)]
#[derive(Debug, Serialize, Deserialize)]
pub struct ArgoCDConfig {
    // #[serde(default)]
    pub version: String,
}

impl Default for ArgoCDConfig {
    fn default() -> Self {
        Self {
            version: String::from("latest"),
        }
    }
}

// #[derive(Serialize, Deserialize, Debug)]
#[derive(Debug, Serialize, Deserialize)]
pub struct PresetsConfig {
    // #[serde(default)]
    pub source: Option<String>,

    // #[serde(default)]
    pub reference: Option<String>,
}

impl Default for PresetsConfig {
    fn default() -> Self {
        Self {
            source: None,
            reference: None,
        }
    }
}

// #[derive(Serialize, Deserialize, Debug)]
#[derive(Debug, Serialize, Deserialize)]
pub struct OptionsConfig {
    // #[serde(default)]
    pub use_application_overlays: bool,

    // #[serde(default)]
    pub use_application_per_project: bool,
}

impl Default for OptionsConfig {
    fn default() -> Self {
        Self {
            use_application_overlays: false,
            use_application_per_project: true,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    // #[serde(default)]
    pub kind: String,

    // #[serde(default)]
    pub kubernetes: KuberentesConfig,

    // #[serde(default)]
    pub argo_cd: ArgoCDConfig,

    // #[serde(default)]
    pub presets: PresetsConfig,

    // #[serde(default)]
    pub options: OptionsConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            kind: String::from("ArgoComposer"),
            kubernetes: KuberentesConfig::default(),
            argo_cd: ArgoCDConfig::default(),
            presets: PresetsConfig::default(),
            options: OptionsConfig::default(),
        }
    }
}

impl Config {
    pub fn from_directory(directory: PathBuf) -> Result<Self, ConfigError> {
        let path = directory.join("argo-composer.yaml");

        if !path.exists() {
            return Ok(Config::default());
        }

        let document: Config = match Yaml::from_path(path) {
            Ok(result) => result,
            Err(_) => {
                return Err(ConfigError::FailedToDeserialize);
            }
        };

        if !document.kind.eq("ArgoComposer") {
            return Err(ConfigError::InvalidConfigurationKind);
        }

        Ok(document)
    }
}
