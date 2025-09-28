mod resources;
mod theme;
mod utils;

use std::{
    env::current_dir,
    fs::{File, create_dir_all, remove_dir_all},
    io::Write,
};

use clap::{Parser, Subcommand};
use cliclack::{input, intro, outro};
use theme::*;

use crate::utils::{
    get_application_directory, get_argo_composer_config, get_project, get_root_application,
};

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
    // Used for updating applications based on a preset template within a project.
    // Apply,
    // Annotate,
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
                        return Err(
                            "Name can only contain lowercase letters and hyphens".to_string()
                        );
                    }

                    if root_name.starts_with('-') || root_name.ends_with('-') {
                        return Err("Name cannot start or end with hyphen".to_string());
                    }

                    if !root_name.split('-').all(|part| !part.is_empty()) {
                        return Err(
                            "Name can only have single hyphen in between the words".to_string()
                        );
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

            File::create(cwd.join(root_application_path))
                .unwrap()
                .write_all(&[])
                .unwrap();

            File::create(root_application_directory.join("kustomization.yaml"))
                .unwrap()
                .write_all(&[])
                .unwrap();

            outro(format!(
                "Initialized root application called `{}`",
                root_application_name
            ))
            .unwrap();
        }
        Commands::Add { command } => match command {
            Resources::Project => {
                intro("Adding new project").unwrap();

                if composer_config.is_none() {
                    terminate_with_message(ARGO_COMPOSER_NOT_INITIALIZED);
                }

                let root_application = get_root_application().unwrap();
                let root_application_projects_directory =
                    root_application.projects_directory.clone();
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

                File::create(project_directory.join(project_filename))
                    .unwrap()
                    .write_all(&[])
                    .unwrap();

                File::create(project_apps_directory.join("kustomization.yaml"))
                    .unwrap()
                    .write_all(&[])
                    .unwrap();

                outro(format!(
                    "Created project `{}` in root application `{}`",
                    project_name, root_application.name
                ))
                .unwrap();
            }
            Resources::Application => {
                intro("Adding application to a existing project").unwrap();

                if composer_config.is_none() {
                    terminate_with_message(ARGO_COMPOSER_NOT_INITIALIZED);
                }

                let root_application = get_root_application().unwrap();
                let project = get_project(root_application.projects_directory).unwrap();
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
        },
        Commands::Remove { command } => match command {
            Resources::Project => {
                intro("Removing project").unwrap();

                if composer_config.is_none() {
                    terminate_with_message(ARGO_COMPOSER_NOT_INITIALIZED);
                }

                let root_application = get_root_application().unwrap();
                let project = get_project(root_application.projects_directory).unwrap();

                remove_dir_all(project.directory).unwrap();

                // todo: update kustomization of a projects

                let message = format!("Removed `{}` project", project.name);

                outro(message).unwrap();
            }
            Resources::Application => {
                intro("Removing application from a existing project").unwrap();

                if composer_config.is_none() {
                    terminate_with_message(ARGO_COMPOSER_NOT_INITIALIZED);
                }

                let root_application = get_root_application().unwrap();
                let project = get_project(root_application.projects_directory).unwrap();
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
        },
    };
}
