use std::{env::current_dir, path::PathBuf};

use rust_yaml::Yaml;

use crate::config::Config;

pub struct Context {
    pub cwd: PathBuf,
    pub argo_composer_directory: PathBuf,
    pub config: Option<Config>,
}

impl Context {
    pub fn new() -> Self {
        let cwd = current_dir().unwrap();
        let argo_composer_directory = cwd.join(".argo-composer");

        Self {
            cwd,
            argo_composer_directory: argo_composer_directory.clone(),
            config: Config::from_directory(argo_composer_directory),
        }
    }

    pub fn find_root_applications(&self) -> Vec<Yaml> {
        Vec::new()
    }

    pub fn find_projects(&self) -> Vec<Yaml> {
        Vec::new()
    }
}
