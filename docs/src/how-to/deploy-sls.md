# Deploy cloud scanner as a serverless application

You can deploy cloud scanner as a serverless application (AWS lambda).

The application is build and deployed using the serverless framework (see [serverless-design](../reference/serverless-design.md)).

## Using Linux

1. Install Rust
2. Install nvm, nodejs
3. test serverless package

First, don't forget to clone the repo using
```sh
git clone https://github.com/Boavizta/cloud-scanner.git
cd cloud-scanner
```

### Install Rust

```sh
# Install rust (see https://www.rust-lang.org/tools/install), validate when prompted
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
# Add Linux musl target (needed for cross compilation for lambda)
rustup target add x86_64-unknown-linux-musl

sudo apt update && sudo apt install -y musl-tools musl-dev
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
```

### Configure environment
Everything you need should now be installed, but there is a last step you have to do before deploying. Since the serverless framework automatically deploys the lambda onto your AWS instance without you having to upload a zip file or anything, you have to give it access to it by giving it an access key (see [this guide](https://www.serverless.com/framework/docs/providers/aws/guide/credentials) for more informations on how to generate an access keys).

```sh
serverless config credentials \
  --provider aws \
  --key YOURACCESSKEY \
  --secret YOURSECRETKEY
```

Optionally, you can config a private instance of Boaviztapi by setting the environment variable `BOAVIZTA_API_URL` so :
```sh
export BOAVIZTA_API_URL="boaviztapi.example.com"
```

If the environment variable is not set, cloud scanner will use the public instance (https://api.boavizta.org) by default.

### Deploy
You should be good to go by now, simply run
```sh
serverless package
serverless deploy
```
and wait for it to be done. You should by now see two lamda functions appear on your AWS instance, Congratulations !

If any error happen, redo those steps carefully and make sure you didn't miss anything before posting a GitHub Issue.

## Using Windows

Tested method to deploy this serverless app from on windows is to use Windows Subsystem For Linux (WSL2).

1. Prerequisite: install WSL2 and  the latest Ubuntu LTS image (22.04).
_If you do not have it yet, you may inspire from [Set up Node.js on WSL 2](https://docs.microsoft.com/en-us/windows/dev-environment/javascript/nodejs-on-wsl)._
1. After WSL is setup, follow Linux instructions above inside the Linux VM.
