# Building CLI

## Linux

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

## Windows: compile cloud-scanner on wsl2

Tested method to build Rust on windows is to use Windows Subsystem For Linux (WSL2)

Prerequisite: install WSL2 and  the latest Ubuntu LTS image (22.04).
If you do not have it yet, you may inspire from [Set up Node.js on WSL 2](https://docs.microsoft.com/en-us/windows/dev-environment/javascript/nodejs-on-wsl).

Once WSL is setup, you just have to follow Linux instructions above inside the Linux VM.

## Using docker
