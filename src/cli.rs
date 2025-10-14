use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum CreateResource {
    Project,
    #[command(alias = "app")]
    Application {
        #[arg(long)]
        helm: Option<bool>,
    },
}

#[derive(Subcommand, Debug)]
pub enum DeleteResource {
    Project,
    #[command(alias = "app")]
    Application,
}

#[derive(Subcommand, Debug)]
pub enum PresetCommands {
    Add,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initalizes root application
    Init,
    /// Creation of Argo resources
    Create {
        #[command(subcommand)]
        resource: CreateResource,
    },
    /// Deletion of Argo resources
    Delete {
        #[command(subcommand)]
        resource: DeleteResource,
    },
    /// Management of Argo Composer application presets
    Presets {
        #[command(subcommand)]
        command: PresetCommands,
    },
}

#[derive(Parser)]
#[command(subcommand_required = true, arg_required_else_help = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}
