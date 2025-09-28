use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug)]
pub enum CreateResourceCommands {
    Project,
    #[command(alias = "app")]
    Application {
        helm: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum DeleteResourceCommands {
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
        resource: CreateResourceCommands,
    },
    /// Deletion of Argo resources
    Delete {
        #[command(subcommand)]
        resource: DeleteResourceCommands,
    },
    /// Management of Argo Composer application presets
    Presets {
        #[command(subcommand)]
        command: PresetCommands,
    },
}

#[derive(Parser)]
#[command(subcommand_required = true, arg_required_else_help = true)]
pub struct ArgoComposer {
    #[command(subcommand)]
    pub command: Option<Commands>,
}
