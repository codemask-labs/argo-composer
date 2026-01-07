use clap::Parser;

mod argocd_resources;
mod cli;
mod commands;
mod config;
mod context;
mod errors;
mod kustomization_resources;
mod template;

use crate::{
    cli::{Cli, Commands},
    commands::{create_command, delete_command, init_command, presets_command},
    context::Context,
};

fn main() {
    let cli = Cli::parse();
    let context = Context::new();

    match cli.command.unwrap() {
        Commands::Init => init_command(context),
        Commands::Create { resource } => create_command(context, resource),
        Commands::Delete { resource } => delete_command(context, resource),
        Commands::Presets { command } => presets_command(context, command),
    };
}
