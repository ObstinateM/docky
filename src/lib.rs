use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::{
    error::Error,
    process::{self, Command},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub docker_repository: String,
    // autres champs du package.json
}

impl Package {
    pub fn get_name(&self) -> String {
        format!(
            "{}/{}:{}",
            &self.docker_repository, &self.name, &self.version
        )
    }

    pub fn get_name_with_tag(&self, tag: &String) -> String {
        format!("{}/{}:{}", &self.docker_repository, &self.name, tag)
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
        .output()
        .unwrap_or_else(|err| exit_string(&format!("Failed to build: {err}")));
    return package.get_name();
}

pub fn tag(package: &Package, previous_tag: String, new_tag: String) -> String {
    Command::new("docker")
        .arg("tag")
        .arg(package.get_name_with_tag(&previous_tag))
        .arg(package.get_name_with_tag(&new_tag))
        .output()
        .unwrap_or_else(|err| {
            eprintln!("Tag fail: {err}");
            process::exit(1)
        });
    return package.get_name_with_tag(&new_tag);
}

pub fn login_docker(user: String, password: String, registry: String) {
    Command::new("docker")
        .arg("login")
        .arg(user)
        .arg(password)
        .arg(registry)
        .output()
        .unwrap_or_else(|_| exit_string("Failed to login to the registry"));
}

pub fn clean_exit(error: impl Error) -> ! {
    eprintln!("Error: {}", error.to_string());
    process::exit(1);
}

pub fn exit_string(error: &str) -> ! {
    eprintln!("Error: {}", error.to_string());
    process::exit(1);
}
