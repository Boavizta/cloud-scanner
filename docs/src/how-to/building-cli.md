# Building cloud-scanner CLI with Cargo 🦀

## On Linux

Install Rust and linux-musl dependencies.

```sh
# Install rust (see https://www.rust-lang.org/tools/install), validate when prompted
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
# Add Linux musl target (needed for cross compilation for aws lambda)
rustup target add x86_64-unknown-linux-musl

sudo apt update && sudo apt install -y musl-tools musl-dev

# Test a build
cargo build

#  build a release
cargo build --release
```

## On Windows with WSL2

Tested method to build Rust on Windows is to use _Windows Subsystem For Linux_ (WSL2)

1. Prerequisite: install WSL2 and  the latest Ubuntu LTS image (22.04).
_If you do not WSL yet, you may inspire from [Set up Node.js on WSL 2](https://docs.microsoft.com/en-us/windows/dev-environment/javascript/nodejs-on-wsl)._
2. After WSL is setup, follow Linux instructions above inside the Linux VM.

## Using docker

See [Build a local docker image](./docker-guide.md#build-a-local-docker-image)
