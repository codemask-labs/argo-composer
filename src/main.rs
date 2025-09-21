mod resources;
mod theme;
mod utils;

use std::{
    fs::{File, create_dir_all},
    io::Write,
};

use clap::{Parser, Subcommand};
use cliclack::{input, intro, outro, select};
use theme::*;
use utils::get_project_list;
// use k8s_openapi::api::

use crate::utils::{ProjectPreset, get_argo_composer_config, get_project_presets};

const ARGO_COMPOSER_NOT_INITIALIZED: &str = "Argo composer not initialized\nPlease make sure you have initialized the project with `argo-composer init` command.";

#[derive(Subcommand, Debug)]
enum Resources {
    Project,
    #[command(alias = "app")]
    Application,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Add {
        #[command(subcommand)]
        command: Resources,
    },
    Remove {
        #[command(subcommand)]
        command: Resources,
    },
    /// Used for updating applications based on a preset template within a project.
    Apply,
    Annotate,
}

#[derive(Parser)]
#[command(subcommand_required = true, arg_required_else_help = true)]
struct ArgoComposer {
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() {
    let cli = ArgoComposer::parse();
    let composer_config = get_argo_composer_config();

    match cli.command.unwrap() {
        Commands::Init => {
            intro("Initializing argo composer default profile").unwrap();

            if composer_config.is_some() {
                terminate_with_message("Argo composer already initialized");
            }
        }
        Commands::Add { command } => match command {
            Resources::Project => {
                intro("Adding new project").unwrap();

                if composer_config.is_none() {
                    terminate_with_message(ARGO_COMPOSER_NOT_INITIALIZED);
                }

                let composer_config = composer_config.unwrap();
                let project_dir = composer_config.root_directory.join("projects");
                let project_dir_clone = composer_config.root_directory.join("projects");
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

                        if project_dir_clone.join(project_name).exists() {
                            return Err("Project already exists".to_string());
                        }

                        Ok(())
                    })
                    .interact()
                    .unwrap();

                println!("Adding project in: {:?}", project_dir.join(&project_name));
            }
            Resources::Application => {
                intro("Adding application to a existing project").unwrap();

                if composer_config.is_none() {
                    terminate_with_message(ARGO_COMPOSER_NOT_INITIALIZED);
                }

                let composer_config = composer_config.unwrap();
                let project_dir = composer_config.root_directory.join("projects");
                let project_list = get_project_list(project_dir.to_str().unwrap());
                let project_items: Vec<(String, String, String)> = project_list
                    .into_iter()
                    .map(|project| (project.name.clone(), project.name.clone(), String::new()))
                    .collect();

                if project_items.is_empty() {
                    terminate_with_message(
                        "No projects found\nPlease add a project first with `argo-composer add project` command.",
                    );
                }

                let selected_project = select("Select a project")
                    .items(project_items.as_slice())
                    .interact()
                    .unwrap();

                let project_apps = project_dir.join(&selected_project).join("apps");
                let project_presets = project_dir.join(&selected_project).join(".presets");
                let project_presets = get_project_presets(project_presets.to_str().unwrap());
                let preset_items: Vec<(ProjectPreset, String, String)> = project_presets
                    .iter()
                    .map(|(preset_name, preset)| {
                        (preset.clone(), preset_name.clone(), String::new())
                    })
                    .collect();

                let selected_preset = select("Select a preset")
                    .items(preset_items.as_slice())
                    .interact()
                    .unwrap();

                let project_apps_validate_path = project_apps.clone();
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
                            return Err(
                                "Application name cannot start or end with hyphens".to_string()
                            );
                        }

                        if !application_name.split('-').all(|part| !part.is_empty()) {
                            return Err(
                                "Application name can only have single hyphen in between the words"
                                    .to_string(),
                            );
                        }

                        if project_apps_validate_path.join(application_name).exists() {
                            return Err("Application already exists".to_string());
                        }

                        Ok(())
                    })
                    .interact()
                    .unwrap();

                let application_directory = project_apps.join(&application_name);
                let application_resources_directory = application_directory.join("resources");

                create_dir_all(&application_directory).unwrap();
                create_dir_all(&application_resources_directory).unwrap();

                File::create(application_directory.join("application.yaml"))
                    .unwrap()
                    .write_all(b"hello world\n")
                    .unwrap();

                File::create(application_resources_directory.join("kustomization.yaml"))
                    .unwrap()
                    .write_all(b"resources: []\n")
                    .unwrap();

                outro(format!(
                    "Created application `{application_name}` in `{selected_project}` project"
                ))
                .unwrap();

                // println!("Apps dir: {}", project_apps.to_str().unwrap());
                // println!("Presets dir: {}", project_presets.to_str().unwrap());
            }
        },
        Commands::Remove { command } => match command {
            Resources::Project => {
                intro("Removing project").unwrap();

                if composer_config.is_none() {
                    terminate_with_message(ARGO_COMPOSER_NOT_INITIALIZED);
                }
            }
            Resources::Application => {
                intro("Removing application from a existing project").unwrap();

                if composer_config.is_none() {
                    terminate_with_message(ARGO_COMPOSER_NOT_INITIALIZED);
                }
            }
        },
        Commands::Apply => {
            intro("Applying application preset within a project").unwrap();

            if composer_config.is_none() {
                terminate_with_message(ARGO_COMPOSER_NOT_INITIALIZED);
            }
        }
        Commands::Annotate => {
            intro("Applying annotations to a application within a project").unwrap();

            if composer_config.is_none() {
                terminate_with_message(ARGO_COMPOSER_NOT_INITIALIZED);
            }
        }
    };
}
