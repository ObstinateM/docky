mod cli;
use clap::Parser;
use cli::{Cli, Commands};
use docky::{build, Package};

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
        Commands::Publish { update } => {
            println!("{:?}", update);
        }
        Commands::Tag { version } => {
            println!("{}", version);
        }
    }
}
