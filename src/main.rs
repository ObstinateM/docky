mod cli;
use clap::Parser;
use cli::{Cli, Commands};
use docky::{build, publish, tag, Package};

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
                    docky: package.docky.clone(),
                });
            }
            println!("Successfully builded.");
        }
        Commands::Config {} => {
            todo!("Should ask for information and add it to config if not exist? (may be remove)")
        }
        Commands::Publish { update } => {
            if let Some(new_version) = update {
                println!("{:?}", new_version);
            }

            build(&package);
            tag(&package, &package.version, &"latest".to_string());
            publish(&package, None);
        }
        Commands::Tag { version } => {
            println!("{}", version);
        }
    }
}
