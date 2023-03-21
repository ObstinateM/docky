use clap::{Parser, Subcommand};
use docky::{build, Package};

#[derive(Debug, Parser)]
#[command(name = "docky")]
#[command(about = "A tool to be blazingly fast with docker", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Create the config file
    Config {},

    /// Build the image
    Build {
        #[arg(default_value = "None")]
        version: String,
    },

    /// Build, tag (version & latest) and publish
    Publish {},

    /// Rename the image
    Tag {
        #[arg(default_value = "latest")]
        version: String,
    },
}

fn main() {
    let args = Cli::parse();
    let package = docky::read_package();
    println!("{:?}", package);

    match args.command {
        Commands::Build { version } => {
            if version == "None" {
                build(&package);
            } else {
                build(&Package {
                    name: package.name.clone(),
                    version,
                    docker_repository: package.docker_repository.clone(),
                });
            }
            println!("Successfully builded.");
        }
        Commands::Config {} => todo!(),
        Commands::Publish {} => todo!(),
        Commands::Tag { version } => {
            println!("{}", version);
        }
    }
}
