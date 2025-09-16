mod theme;
mod utils;

use clap::{Parser, Subcommand};
use cliclack::{intro, select};
use theme::*;
use utils::get_project_list;

use crate::utils::get_argo_composer_config;

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
}

#[derive(Parser)]
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
                intro("Adding project").unwrap();

                if composer_config.is_none() {
                    terminate_with_message(ARGO_COMPOSER_NOT_INITIALIZED);
                }
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

                // let selected_preset =
                //     select("Select application preset").items(application_presets);

                println!("Apps dir: {}", project_apps.to_str().unwrap());
                println!("Presets dir: {}", project_presets.to_str().unwrap());
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
    };
}
