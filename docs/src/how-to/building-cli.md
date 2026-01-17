# Building cloud-scanner CLI with Cargo 🦀

Cloud scanner is mostly implemented in Rust. You need to install Rust on your system to build the project from scratch.

## Using a devcontainer

You can build cloud scanner without installing Rust toolchain by using the provided [devcontainer](https://containers.dev/). It is a docker container that already contains the necessary build tools.

In VS code:
- install the devcontainer extension (`ms-vscode-remote.remote-containers)
- from the command palete, select `Dev Container:Reopen in container`

VS code will build the container if needed, run it and attach it's current session to this container. You can then develop in VS code without directly as if you had the build toolchain installed on your system.

## On Linux

Install Rust and linux-musl dependencies.

```sh
# Install rust (see https://www.rust-lang.org/tools/install), validate when prompted
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
# Add Linux musl target (needed for cross compilation for aws lambda)
rustup target add x86_64-unknown-linux-musl

sudo apt update && sudo apt install -y musl-tools musl-dev

# Build cloud-scanner
cargo build

# Build a release
cargo build --release

# [Optional] To work on documentation
# Install globally mdBook and some preprocessors(to work on documentation)
cargo install mdbook mdbook-mermaid mdbook-linkcheck
# Serve documentation
cd docs
mdbook serve
```

## On Windows with WSL2

Tested method to build Rust on Windows is to use _Windows Subsystem For Linux_ (WSL2)

1. Prerequisite: install WSL2 and the latest Ubuntu LTS image (22.04).
_If you are not familiar with WSL, you may inspire from [Set up Node.js on WSL 2](https://docs.microsoft.com/en-us/windows/dev-environment/javascript/nodejs-on-wsl)._
2. After WSL is setup, follow Linux instructions above inside the Linux VM.

## Using docker

See [Build a local docker image](./docker-guide.md#build-a-local-docker-image)
