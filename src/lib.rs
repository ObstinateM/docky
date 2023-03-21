use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::process::ExitStatus;
use std::{
    error::Error,
    process::{self, Command},
};

trait CommandExt {
    fn exit_if_error(&mut self, exit: &str) -> ExitStatus;
}

impl CommandExt for Command {
    fn exit_if_error(&mut self, exit: &str) -> ExitStatus {
        match self.status() {
            Ok(status) => {
                if status.success() {
                    status
                } else {
                    exit_string(exit);
                }
            }
            Err(_) => {
                exit_string(exit);
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub docky: DockerRepository,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DockerRepository {
    pub url: String,
    pub r#type: RepositoryType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum RepositoryType {
    Azure,
    Aws,
    Docker,
}

impl Package {
    pub fn get_name(&self) -> String {
        format!("{}/{}:{}", &self.docky.url, &self.name, &self.version)
    }

    pub fn get_name_with_tag(&self, tag: &String) -> String {
        format!("{}/{}:{}", &self.docky.url, &self.name, tag)
    }
}

pub enum Update {
    Major,
    Minor,
    Patch,
}

pub fn read_package() -> Package {
    let mut file =
        File::open("package.json").unwrap_or_else(|_| exit_string("Cannot find package.json"));
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .unwrap_or_else(|_| exit_string("Cannot read package.json"));
    let package: Package =
        serde_json::from_str(&contents).unwrap_or_else(|err| exit_string(&err.to_string()));
    return package;
}

pub fn build(package: &Package) -> String {
    Command::new("docker")
        .arg("build")
        .arg("-t")
        .arg(&package.get_name())
        .arg(".")
        .exit_if_error("Failed to build");

    return package.get_name();
}

pub fn tag(package: &Package, previous_tag: &String, new_tag: &String) -> String {
    Command::new("docker")
        .arg("tag")
        .arg(package.get_name_with_tag(&previous_tag))
        .arg(package.get_name_with_tag(&new_tag))
        .exit_if_error("Failed to tag");
    return package.get_name_with_tag(&new_tag);
}

pub fn publish(package: &Package, version: Option<String>) {
    // Log to the docker registry
    match package.docky.r#type {
        RepositoryType::Azure => {}
        RepositoryType::Aws => {}
        RepositoryType::Docker => login_docker(
            &"user".to_string(),
            &"password".to_string(),
            &package.docky.url,
        ),
    };

    // Push the image
    Command::new("docker")
        .arg("push")
        .arg(package.get_name_with_tag(&version.unwrap_or("latest".to_owned())))
        .exit_if_error("Failed to publish");
}

fn login_docker(user: &String, password: &String, registry: &String) {
    Command::new("docker")
        .arg("login")
        .arg("-u")
        .arg(user)
        .arg("-p")
        .arg(password)
        .arg(registry)
        .exit_if_error("Failed to login");
}

pub fn clean_exit(error: impl Error) -> ! {
    eprintln!("Error: {}", error.to_string());
    process::exit(1);
}

pub fn exit_string(error: &str) -> ! {
    eprintln!("Error: {}", error.to_string());
    process::exit(1);
}
