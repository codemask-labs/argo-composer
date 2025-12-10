use std::{env::current_dir, path::PathBuf};

use yaml::Yaml;

use crate::config::Config;

pub struct Context {
    pub cwd: PathBuf,
    pub argo_composer_directory: PathBuf,
    pub config: Yaml<Config>,
}

impl Context {
    pub fn new() -> Self {
        let cwd = current_dir().unwrap();
        let argo_composer_directory = cwd.join(".argo-composer");
        let config = Config::from_directory(argo_composer_directory.clone()).unwrap();

        println!("config :: {:?}", config);

        Self {
            cwd,
            argo_composer_directory: argo_composer_directory,
            config: config,
        }
    }

    // pub fn find_root_applications(&self) -> Vec<Yaml<Application>> {
    //     Vec::new()
    // }

    // pub fn find_projects(&self) -> Vec<Yaml<Application>> {
    //     Vec::new()
    // }
}
