use std::{
    collections::HashMap,
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum PresetInputType {
    String,
    Number,
    Boolean,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PresetInput {
    ty: PresetInputType,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProjectPreset {
    pub name: String,
    pub description: Option<String>,
    pub template_directory: PathBuf,
    pub inputs: Vec<PresetInput>,
}

pub fn get_project_presets(project_presets_dir: &str) -> HashMap<String, ProjectPreset> {
    let cwd = current_dir().unwrap();
    let profile_presets_dir = Path::new(&cwd)
        .join(".argo-composer")
        .join("default")
        .join("presets");
    let profile_presets = std::fs::read_dir(profile_presets_dir).unwrap();
    let project_presets = std::fs::read_dir(project_presets_dir).unwrap();

    let mut result = HashMap::new();

    let presets = profile_presets
        .into_iter()
        .chain(project_presets.into_iter());

    for preset in presets.into_iter() {
        let preset_path = preset.unwrap().path();

        if !preset_path.is_dir() {
            continue;
        }

        let preset_config_path = preset_path.join("preset.yaml");
        let preset_config_file = std::fs::File::open(preset_config_path);

        if preset_config_file.is_err() {
            continue;
        }

        let mut preset_config_contents = String::new();

        if let Err(_) = preset_config_file
            .unwrap()
            .read_to_string(&mut preset_config_contents)
        {
            continue;
        }

        let preset_config_docs = YamlLoader::load_from_str(&preset_config_contents);

        if preset_config_docs.is_err() {
            continue;
        }

        let docs = preset_config_docs.unwrap();
        let name = docs[0]["name"].as_str().unwrap().to_string();

        result.insert(
            name.clone(),
            ProjectPreset {
                name,
                description: docs[0]["description"].as_str().map(|s| s.to_string()),
                template_directory: preset_path.join("template"),
                inputs: Vec::new(),
            },
        );
    }

    result
}
