use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "docky")]
#[command(about = "A tool to be blazingly fast with docker", long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Create the config file
    Config {},

    /// Build the image
    Build {
        #[arg(default_value = "None")]
        version: String,
    },

    /// Build, tag (version & latest) and publish
    Publish {
        #[command(subcommand)]
        update: Option<Update>,
    },

    /// Rename the image
    Tag {
        #[arg(default_value = "latest")]
        version: String,
    },
}

#[derive(Debug, Clone, clap::Subcommand)]
pub enum Update {
    Major,
    Minor,
    Patch,
    None,
}
