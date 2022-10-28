# Quick start run CLI using docker 🐳

No installation needed, you will run a public docker image of cloud-scanner CLI.

## Pre-requisites

- Docker.
- A working AWS account (and you AWS CLI profile already configured)

## Run cloud-scanner cli

```sh
docker pull ghcr.io/boavizta/cloud-scanner-cli:latest
docker run -it ghcr.io/boavizta/cloud-scanner-cli:latest --help

# Ensure you have a working AWS profile setup locally (like you would do for AWS CLI)
# Note
# - we map local credentials on the container (-v)
# - we force a using 'myprofile' profile by setting the AWS_PROFILE environment variable with -e flag
# - the -it flag is optional, only purpose is to get colored output if any

# Just list instances
docker run -it -v $HOME/.aws/credentials:/root/.aws/credentials:ro -e AWS_PROFILE='myprofile' ghcr.io/boavizta/cloud-scanner-cli:latest list-instances

# List instances and standard impacts (for 10 hours of use)
docker run -it -v $HOME/.aws/credentials:/root/.aws/credentials:ro -e AWS_PROFILE='myprofile' ghcr.io/boavizta/cloud-scanner-cli:latest standard --hours-use-time 10
```

⚠ This method of passing credentials is not secure nor very practical. In a production setup on AWS, you should rather rely on the role of the instance that execute the container to manage authentication of the cli.

Alternatively you may build and use a local docker image, See [build a local docker image](../how-to/docker-guide.md#build-a-local-docker-image)
