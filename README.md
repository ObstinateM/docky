# Docky CLI

Docky is a command-line tool (CLI) to simplify the use of Docker. It allows you to factorize common Docker commands by retrieving information from a `package.json` file.

## Installation

To install Docky, you must have Rust and Cargo installed on your system. Then, run the following command:

```bash
cargo install --git https://github.com/ObstinateM/docky-cli
```

This will install Docky on your system.

## Usage

To use Docky, you can run the following command:

```bash
docky --help
```

This will display help for all available commands.

## Configuration

To use Docky, you must have a package.json file at the root of your project. This file must contain the following information:

```json
{
  "name": "my-project",
  "version": "1.0.0",
  "docky": {
    "type": "Docker", // Or Azure, Aws
    "url": "docker.obstinate.fr"
  }
}
```

## Commands

Docky supports the following commands:

`docky build`

This command builds your Docker image using the information from your package.json file.

You can specify a version for your image using the --version option. If you don't specify a version, the latest version will be used.

TODO: Add the others commands

## Contribution

Contributions are welcome! If you want to contribute to this project, please open a pull request.
