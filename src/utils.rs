use std::{
    env::current_dir,
    io::Read,
    path::{Path, PathBuf},
};

use cliclack::select;
use yaml_rust2::YamlLoader;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Project {
    pub name: String,
    pub directory: PathBuf,
    pub apps_directory: Option<PathBuf>,
}

impl Project {
    pub fn new() -> Project {
        Project {
            name: String::new(),
            directory: PathBuf::new(),
            apps_directory: None,
        }
    }

    pub fn get_apps_directory(&self) -> PathBuf {
        if self.apps_directory.is_none() {
            return self.directory.join("apps");
        }

        self.apps_directory.clone().unwrap()
    }
}

pub fn find_projects(directory: PathBuf) -> Vec<Project> {
    let mut result = Vec::new();
    let projects = std::fs::read_dir(directory).unwrap();

    for project in projects {
        let project_directory = project.unwrap().path();

        if !project_directory.is_dir() {
            continue;
        }

        let project_name = project_directory.file_name().unwrap().to_str().unwrap();
        let project_path = project_directory.join(format!("{}.yaml", project_name));

        if !project_path.exists() {
            continue;
        }

        let project_contents = std::fs::read_to_string(project_path).unwrap();
        let project_documents = YamlLoader::load_from_str(&project_contents).unwrap();

        if project_documents.is_empty()
            || project_documents.iter().all(|doc| {
                let kind = doc["kind"].as_str();

                kind.is_none() || kind.unwrap() != "AppProject"
            })
        {
            continue;
        }

        let mut project = Project::new();

        for document in project_documents {
            let kind = document["kind"].as_str();

            if kind.is_none() {
                continue;
            }

            let kind = kind.unwrap();

            if kind == "AppProject" {
                let name = document["metadata"]["name"].as_str().unwrap().to_string();

                project.name = name;
                project.directory = project_directory.clone();

                continue;
            }

            if kind == "Application" {
                let cwd = current_dir().unwrap();
                let source_path = document["spec"]["source"]["path"].as_str().unwrap();

                project.apps_directory = Some(cwd.join(source_path));

                continue;
            }
        }

        result.push(project);
    }

    result
}

pub fn get_project(directory: PathBuf) -> Option<Project> {
    let projects = find_projects(directory);

    if projects.is_empty() {
        return None;
    }

    let project = match projects.len() {
        1 => &projects[0],
        _ => {
            let projects: Vec<(&Project, String, String)> = projects
                .iter()
                .map(|project| (project, project.name.clone(), String::new()))
                .collect();

            select("Select project")
                .items(&projects)
                .interact()
                .unwrap()
        }
    };

    Some(project.clone())
}

pub struct ArgoComposerConfig {}

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
        Ok(_) => {
            // let root_directory = docs[0]["root-directory"].as_str().unwrap();
            // let root_directory = Path::new(&cwd).join(root_directory);

            Some(ArgoComposerConfig {})
        }
        Err(_) => None,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RootApplication {
    pub name: String,
    pub projects_directory: PathBuf,
    pub repository_url: String,
}

pub fn find_root_applications() -> Vec<RootApplication> {
    let cwd = current_dir().unwrap();
    let root_applications = std::fs::read_dir(&cwd).unwrap();

    let mut result = Vec::new();

    for root_application in root_applications {
        let root_application_path = root_application.unwrap().path();

        if !root_application_path
            .file_name()
            .and_then(|n| n.to_str())
            .is_some_and(|s| s.ends_with(".root-app.yaml"))
        {
            continue;
        }

        let contents = std::fs::read_to_string(root_application_path).unwrap();
        let applications = YamlLoader::load_from_str(&contents).unwrap();

        for application in applications {
            // todo: validate `application` against a Application CRD Schema
            let name = application["metadata"]["name"]
                .as_str()
                .unwrap()
                .to_string();
            let projects_directory = application["spec"]["source"]["path"]
                .as_str()
                .unwrap()
                .to_string();
            let repository_url = application["spec"]["source"]["repoURL"]
                .as_str()
                .unwrap()
                .to_string();

            result.push(RootApplication {
                name,
                projects_directory: Path::new(&cwd).join(projects_directory),
                repository_url,
            });
        }
    }

    result
}

pub fn get_root_application() -> Option<RootApplication> {
    let root_applications = find_root_applications();

    if root_applications.is_empty() {
        return None;
    }

    let root_application = match root_applications.len() {
        1 => &root_applications[0],
        _ => {
            let root_applications: Vec<(&RootApplication, String, String)> = root_applications
                .iter()
                .map(|root_application| {
                    (
                        root_application,
                        root_application.name.clone(),
                        String::new(),
                    )
                })
                .collect();

            select("Select root application")
                .items(&root_applications)
                .interact()
                .unwrap()
        }
    };

    Some(root_application.clone())
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Application {
    pub name: String,
    pub directory: PathBuf,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ApplicationDirectory {
    pub name: String,
    pub directory: PathBuf,
    pub applications: Vec<Application>,
}

pub fn find_application_directories(directory: PathBuf) -> Vec<ApplicationDirectory> {
    let mut result = Vec::new();

    for application in std::fs::read_dir(directory).unwrap() {
        let application_directory = application.unwrap().path();

        if !application_directory.is_dir() {
            continue;
        }

        let application_path = application_directory.join("application.yaml");

        if !application_path.exists() {
            continue;
        }

        let contents = std::fs::read_to_string(application_path).unwrap();
        let applications = YamlLoader::load_from_str(&contents).unwrap();

        if !applications.iter().any(|application| {
            let kind = application["kind"].as_str();

            kind.is_some() && kind.unwrap() == "Application"
        }) {
            continue;
        }

        result.push(ApplicationDirectory {
            name: application_directory
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap()
                .to_string(),
            directory: application_directory,
            applications: Vec::new(),
        });
    }

    result
}

pub fn get_application_directory(directory: PathBuf) -> Option<ApplicationDirectory> {
    let application_directories = find_application_directories(directory);

    if application_directories.is_empty() {
        return None;
    }

    let application_directory = match application_directories.len() {
        1 => &application_directories[0],
        _ => {
            let application_directories: Vec<(&ApplicationDirectory, String, String)> =
                application_directories
                    .iter()
                    .map(|application_directory| {
                        (
                            application_directory,
                            application_directory.name.clone(),
                            String::new(),
                        )
                    })
                    .collect();

            select("Select application")
                .items(&application_directories)
                .interact()
                .unwrap()
        }
    };

    Some(application_directory.clone())
}
