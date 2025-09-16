use std::{
    env::current_dir,
    io::Read,
    path::{Path, PathBuf},
};

use yaml_rust2::YamlLoader;

#[derive(Debug)]
pub struct Project {
    pub name: String,
}

pub fn get_project_list(project_dir: &str) -> Vec<Project> {
    let mut result = Vec::new();
    let directory_items = std::fs::read_dir(project_dir).unwrap();

    for item in directory_items {
        let item = item.unwrap().path();

        if !item.is_dir() {
            continue;
        }

        result.push(Project {
            name: item.file_name().unwrap().to_str().unwrap().to_string(),
        });
    }

    result
}

pub struct ArgoComposerConfig {
    pub root_directory: PathBuf,
}

pub fn get_argo_composer_config() -> Option<ArgoComposerConfig> {
    let cwd = current_dir().unwrap();
    let config_path = Path::new(&cwd).join(".argo-composer/argo-composer.yaml");
    let config_file = std::fs::File::open(config_path);

    if config_file.is_err() {
        return None;
    }

    let mut contents = String::new();

    if let Err(_) = config_file.unwrap().read_to_string(&mut contents) {
        return None;
    }

    match YamlLoader::load_from_str(&contents) {
        Ok(docs) => {
            let root_directory = docs[0]["root-directory"].as_str().unwrap();
            let root_directory = Path::new(&cwd).join(root_directory);

            Some(ArgoComposerConfig { root_directory })
        }
        Err(_) => None,
    }
}
