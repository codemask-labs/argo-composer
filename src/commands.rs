use std::{
    collections::BTreeMap,
    env::current_dir,
    fs::{File, create_dir_all, remove_dir_all},
    io::Write,
};

use cliclack::{confirm, input, intro, outro};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;

use crate::{
    cli::{CreateResourceCommands, DeleteResourceCommands, PresetCommands},
    messages::{ARGO_COMPOSER_NOT_INITIALIZED, terminate_with_message},
    resources::{
        AppProject, AppProjectSpec, Application, ApplicationDestination, ApplicationSource,
        ApplicationSpec, GroupKind, Kustomization, SyncPolicy,
    },
    utils::{
        ProjectDirectory, get_application_directory, get_argo_composer_config,
        get_project_directory, get_root_application,
    },
};

pub fn init_command() {
    intro("Initializing argo composer root projects").unwrap();

    let cwd = current_dir().unwrap();
    let argo_composer_directory = cwd.join(".argo-composer");
    let root_application_name: String = input("Root application name")
        .default_input("projects")
        .validate_on_enter(|root_name: &String| {
            if root_name.is_empty() {
                return Err("Name cannot be empty".to_string());
            }

            if !root_name
                .chars()
                .all(|c| c.is_ascii_lowercase() || c == '-')
            {
                return Err("Name can only contain lowercase letters and hyphens".to_string());
            }

            if root_name.starts_with('-') || root_name.ends_with('-') {
                return Err("Name cannot start or end with hyphen".to_string());
            }

            if !root_name.split('-').all(|part| !part.is_empty()) {
                return Err("Name can only have single hyphen in between the words".to_string());
            }

            let cwd = current_dir().unwrap();

            if std::fs::read_dir(cwd.join(root_name)).is_ok() {
                return Err("Root application already exists".to_string());
            }

            Ok(())
        })
        .interact()
        .unwrap();

    let argo_composer_config_path = argo_composer_directory.join("argo-composer.yaml");

    if !argo_composer_config_path.exists() {
        // Handling creation of argo composer configuration, if it doesn't exists
        // todo: ask about kubernetes version and argo-cd version

        create_dir_all(&argo_composer_directory).unwrap();

        File::create(argo_composer_config_path)
            .unwrap()
            .write_all(&[])
            .unwrap();
    }

    let root_application_filename = format!("{}.root-app.yaml", root_application_name);
    let root_application_path = cwd.join(root_application_filename);
    let root_application_directory = cwd.join(&root_application_name);

    create_dir_all(&root_application_directory).unwrap();

    let application_resource = Application {
        metadata: Some(ObjectMeta {
            name: Some(root_application_name.clone()),
            namespace: Some(String::from("argocd")),
            finalizers: Some([String::from("resources-finalizer.argocd.argoproj.io")].to_vec()),
            annotations: Some(BTreeMap::from([(
                String::from("argocd.argoproj.io/manifest-generate-paths"),
                String::from("."),
            )])),
            ..Default::default()
        }),
        spec: ApplicationSpec {
            project: String::from("default"),
            source: Some(ApplicationSource {
                // todo: ask about those values
                repo_url: Some(String::from("https://github.com/example/argocd-resources")),
                target_revision: Some(String::from("main")),
                path: Some(root_application_name.clone()),
                ..Default::default()
            }),
            destination: ApplicationDestination {
                server: Some(String::from("https://kubernetes.default.svc")),
                namespace: Some(String::from("argocd")),
                ..Default::default()
            },
            ..Default::default()
        },
        ..Default::default()
    };

    File::create(cwd.join(root_application_path))
        .unwrap()
        .write_all(
            serde_yaml::to_string(&application_resource)
                .unwrap()
                .as_bytes(),
        )
        .unwrap();

    let kustomization_resource = Kustomization {
        ..Default::default()
    };

    File::create(root_application_directory.join("kustomization.yaml"))
        .unwrap()
        .write_all(
            serde_yaml::to_string(&kustomization_resource)
                .unwrap()
                .as_bytes(),
        )
        .unwrap();

    outro(format!(
        "Initialized root application called `{}`",
        root_application_name
    ))
    .unwrap();
}

pub fn create_command(command: CreateResourceCommands) {
    let composer_config = get_argo_composer_config();

    match command {
        CreateResourceCommands::Project => {
            intro("Adding new project").unwrap();

            if composer_config.is_none() {
                terminate_with_message(ARGO_COMPOSER_NOT_INITIALIZED);
            }

            let root_application = get_root_application().unwrap();
            let root_application_projects_directory = root_application.projects_directory.clone();
            let project_name: String = input("Project name")
                .validate_on_enter(move |project_name: &String| {
                    if project_name.is_empty() {
                        return Err("Project name cannot be empty".to_string());
                    }

                    if !project_name
                        .chars()
                        .all(|c| c.is_ascii_lowercase() || c == '-')
                    {
                        return Err(
                            "Project name can only contain lowercase letters and hyphens"
                                .to_string(),
                        );
                    }

                    if project_name.starts_with('-') || project_name.ends_with('-') {
                        return Err("Project name cannot start or end with hyphens".to_string());
                    }

                    if !project_name.split('-').all(|part| !part.is_empty()) {
                        return Err(
                            "Project name can only have single hyphen in between the words"
                                .to_string(),
                        );
                    }

                    if root_application_projects_directory
                        .join(project_name)
                        .exists()
                    {
                        return Err("Project already exists".to_string());
                    }

                    Ok(())
                })
                .interact()
                .unwrap();

            let project_filename = format!("{}.yaml", &project_name);
            let project_directory = root_application.projects_directory.join(&project_name);
            let project_apps_directory = project_directory.join("apps");

            create_dir_all(&project_apps_directory).unwrap();
            create_dir_all(&project_directory).unwrap();

            let application_resource = Application {
                metadata: Some(ObjectMeta {
                    name: Some(project_name.clone()),
                    namespace: Some(String::from("argocd")),
                    finalizers: Some(
                        [String::from("resources-finalizer.argocd.argoproj.io")].to_vec(),
                    ),
                    ..Default::default()
                }),
                ..Default::default()
            };

            let project_resource = AppProject {
                metadata: Some(ObjectMeta {
                    name: Some(project_name.clone()),
                    namespace: Some(String::from("argocd")),
                    finalizers: Some(
                        [String::from("resources-finalizer.argocd.argoproj.io")].to_vec(),
                    ),
                    ..Default::default()
                }),
                spec: AppProjectSpec {
                    source_repos: Vec::from([String::from(
                        "https://github.com/example/argocd-resources",
                    )]),
                    destinations: Vec::from([ApplicationDestination {
                        server: Some(String::from("*")),
                        namespace: Some(String::from("*")),
                        ..Default::default()
                    }]),
                    cluster_resource_whitelist: Vec::from([GroupKind {
                        kind: String::from("*"),
                        group: Some(String::from("*")),
                        ..Default::default()
                    }]),
                    namespace_resource_whitelist: Vec::from([GroupKind {
                        kind: String::from("*"),
                        group: Some(String::from("*")),
                        ..Default::default()
                    }]),
                    ..Default::default()
                },
                ..AppProject::default()
            };

            File::create(project_directory.join(project_filename))
                .unwrap()
                .write_all(serde_yaml::to_string(&project_resource).unwrap().as_bytes())
                .unwrap();

            let kustomization_resource = Kustomization {
                ..Default::default()
            };

            File::create(project_apps_directory.join("kustomization.yaml"))
                .unwrap()
                .write_all(
                    serde_yaml::to_string(&kustomization_resource)
                        .unwrap()
                        .as_bytes(),
                )
                .unwrap();

            outro(format!(
                "Created project `{}` in root application `{}`",
                project_name, root_application.name
            ))
            .unwrap();
        }
        // todo: handle creating helm application
        CreateResourceCommands::Application { helm: _helm } => {
            intro("Adding application to project").unwrap();

            if composer_config.is_none() {
                terminate_with_message(ARGO_COMPOSER_NOT_INITIALIZED);
            }

            let root_application = get_root_application().unwrap();
            let project = get_project_directory(root_application.projects_directory).unwrap();
            let project_apps = project.get_apps_directory();

            let application_name: String = input("Application name")
                .validate_on_enter(move |application_name: &String| {
                    if application_name.is_empty() {
                        return Err("Application name cannot be empty".to_string());
                    }

                    if !application_name
                        .chars()
                        .all(|c| c.is_ascii_lowercase() || c == '-')
                    {
                        return Err(
                            "Application name can only contain lowercase letters and hyphens"
                                .to_string(),
                        );
                    }

                    if application_name.starts_with('-') || application_name.ends_with('-') {
                        return Err("Application name cannot start or end with hyphens".to_string());
                    }

                    if !application_name.split('-').all(|part| !part.is_empty()) {
                        return Err(
                            "Application name can only have single hyphen in between the words"
                                .to_string(),
                        );
                    }

                    if project_apps.join(application_name).exists() {
                        return Err("Application already exists".to_string());
                    }

                    Ok(())
                })
                .interact()
                .unwrap();

            let application_directory = project.get_apps_directory().join(&application_name);

            create_dir_all(&application_directory).unwrap();

            // todo: add application based on a project or composer preset

            let message = format!(
                "Created `{}` application in `{}` project",
                application_name, project.name
            );

            outro(message).unwrap();
        }
    };
}

pub fn delete_command(command: DeleteResourceCommands) {
    let composer_config = get_argo_composer_config();

    match command {
        DeleteResourceCommands::Project => {
            intro("Removing project").unwrap();

            if composer_config.is_none() {
                terminate_with_message(ARGO_COMPOSER_NOT_INITIALIZED);
            }

            let root_application = get_root_application().unwrap();
            let project = get_project_directory(root_application.projects_directory).unwrap();

            remove_dir_all(project.directory).unwrap();

            // todo: update kustomization of a projects

            let message = format!("Removed `{}` project", project.name);

            outro(message).unwrap();
        }
        DeleteResourceCommands::Application => {
            intro("Removing application from a existing project").unwrap();

            if composer_config.is_none() {
                terminate_with_message(ARGO_COMPOSER_NOT_INITIALIZED);
            }

            let root_application = get_root_application().unwrap();
            let project = get_project_directory(root_application.projects_directory).unwrap();
            let project_apps = project.get_apps_directory();
            let application_directory = match get_application_directory(project_apps) {
                Some(application_directory) => application_directory,
                None => terminate_with_message("The project has no applications"),
            };

            remove_dir_all(application_directory.directory).unwrap();

            // todo: update kustomization of a project apps

            let message = format!(
                "Removed `{}` application from `{}` project",
                application_directory.name, project.name
            );

            outro(message).unwrap();
        }
    };
}

pub fn presets_command(command: PresetCommands) {
    let composer_config = get_argo_composer_config();

    match command {
        PresetCommands::Add => {
            intro("Adding new preset").unwrap();

            if composer_config.is_none() {
                terminate_with_message(ARGO_COMPOSER_NOT_INITIALIZED);
            }

            let is_project_scope_preset = confirm("Do you want to add project scoped preset?")
                .initial_value(true)
                .interact()
                .unwrap();

            let mut presets_directory = composer_config.unwrap().presets_directory.clone();

            if is_project_scope_preset {
                let root_application = get_root_application().unwrap();
                let projects_directory = root_application.projects_directory;
                let project_directory = match get_project_directory(projects_directory) {
                    Some(project_directory) => project_directory,
                    None => terminate_with_message("Needs at least one project"),
                };

                presets_directory = project_directory.directory.join(".presets");
            }

            let presets_directory_clone = presets_directory.clone();
            let preset_name: String = input("Preset name")
                .default_input("default")
                .validate_on_enter(move |preset_name: &String| {
                    if preset_name.is_empty() {
                        return Err("Preset name cannot be empty".to_string());
                    }

                    if !preset_name
                        .chars()
                        .all(|c| c.is_ascii_lowercase() || c == '-')
                    {
                        return Err("Preset name can only contain lowercase letters and hyphens"
                            .to_string());
                    }

                    if preset_name.starts_with('-') || preset_name.ends_with('-') {
                        return Err("Preset name cannot start or end with hyphens".to_string());
                    }

                    if !preset_name.split('-').all(|part| !part.is_empty()) {
                        return Err(
                            "Preset name can only have single hyphen in between the words"
                                .to_string(),
                        );
                    }

                    if presets_directory_clone.join(preset_name).exists() {
                        return Err("Preset already exists".to_string());
                    }

                    Ok(())
                })
                .interact()
                .unwrap();

            let preset_directory = presets_directory.join(&preset_name);

            create_dir_all(presets_directory).unwrap();
            create_dir_all(preset_directory).unwrap();
        }
    };
}
