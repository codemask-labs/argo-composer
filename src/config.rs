use std::path::PathBuf;

use rust_yaml::Yaml;

pub struct Config {
    pub inner: Option<Yaml>,
    pub common_directory: PathBuf,
    pub presets_directory: PathBuf,
    pub kubernetes_version: String,
    pub argo_cd_version: String,
    pub option_use_application_overlays: bool,
    pub option_use_application_per_project: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            inner: None,
            common_directory: PathBuf::new(),
            presets_directory: PathBuf::new(),
            kubernetes_version: String::from("latest"),
            argo_cd_version: String::from("latest"),
            option_use_application_overlays: false,
            option_use_application_per_project: true,
        }
    }
}

impl Config {
    pub fn from_directory(directory: PathBuf) -> Self {
        let config_path = directory.join("argo-composer.yaml");

        if !config_path.exists() {
            return Self {
                common_directory: directory.join("common"),
                presets_directory: directory.join("presets"),
                ..Default::default()
            };
        }

        todo!()
    }
}
