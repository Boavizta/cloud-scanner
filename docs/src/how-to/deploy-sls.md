# Deploy cloud scanner as a serverless application

You can deploy cloud scanner as a serverless application (AWS lambda).

## Using Linux

1. Install Rust
2. Install nvm, nodejs
3. test serverless package

### Install Rust

```sh
# Install rust (see https://www.rust-lang.org/tools/install), validate when prompted
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
# Add Linux musl target (needed for cross compilation for lambda)
rustup target add x86_64-unknown-linux-musl

sudo apt update && sudo apt install -y musl-tools musl-dev

# Test a build
cargo build
```

### Install node (for serverless deployment)

nvm, node.js and npm

See https://docs.microsoft.com/en-us/windows/dev-environment/javascript/nodejs-on-wsl

```sh
sudo apt-get install curl

# Install nvm
curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.1/install.sh | bash
# Install node
nvm install --lts
```

### Install serverless framework and deps

```sh
npm install -g serverless
npm i
# Test packaging
serverless package
# deploy
serverless deploy
```

## Using Windows

Tested method to deploy this serverless app from on windows is to use Windows Subsystem For Linux (WSL2).

1. Prerequisite: install WSL2 and  the latest Ubuntu LTS image (22.04).
_If you do not have it yet, you may inspire from [Set up Node.js on WSL 2](https://docs.microsoft.com/en-us/windows/dev-environment/javascript/nodejs-on-wsl)._
1. After WSL is setup, follow Linux instructions above inside the Linux VM.
