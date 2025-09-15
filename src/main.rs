mod theme;
mod utils;

use clap::{Parser, Subcommand};
use cliclack::{intro, select};
use theme::*;
use utils::get_project_list;

use crate::utils::get_composer_config;

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
    let composer_config = get_composer_config();

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
                let project_list = get_project_list("./example/projects");

                println!("Root directory: {:?}", composer_config.root_directory);
                println!("Projects: {:#?}", project_list);

                let project = select("Select a project")
                    .items(&[
                        ("project-1", "Project 1", "Project 1 description"),
                        ("project-2", "Project 2", "Project 2 description"),
                    ])
                    .interact()
                    .unwrap();

                println!("Selected project: {}", project);
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
