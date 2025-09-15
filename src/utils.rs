use std::{
    env::current_dir,
    io::Read,
    path::{Path, PathBuf},
};

use yaml_rust2::YamlLoader;

pub fn get_project_list(project_dir: &str) -> Vec<String> {
    let project_list = std::fs::read_dir(project_dir).unwrap();
    project_list
        .map(|res| res.unwrap().path().display().to_string())
        .collect()
}

pub struct ComposerConfig {
    pub root_directory: PathBuf,
}

pub fn get_composer_config() -> Option<ComposerConfig> {
    let cwd = current_dir().unwrap().to_str().unwrap().to_string();
    let config_path = format!("{}/.argo-composer/argo-composer.yaml", cwd);

    match std::fs::File::open(config_path) {
        Ok(mut config_file) => {
            let mut contents = String::new();

            match config_file.read_to_string(&mut contents) {
                Ok(_) => {
                    let docs = YamlLoader::load_from_str(&contents).unwrap();
                    let root_directory = docs[0]["root-directory"].as_str().unwrap().to_string();
                    let root_directory = Path::new(&cwd).join(root_directory);

                    Some(ComposerConfig { root_directory })
                }
                Err(_) => None,
            }
        }
        Err(_) => None,
    }
}
