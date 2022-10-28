# Boavizta cloud-scanner 📡

Returns Boavizta impact data corresponding to your AWS Cloud usage.

As a command line or serverless application, cloud-scanner analyses your EC2 instances and returns impacts metrics using the [Boavizta API](https://github.com/Boavizta/boaviztapi/).

Cloud-scanner can be automated to produce metrics at regular interval and monitor your impacts in a dashboard.

![A example dashboard rendering cloud scanner metrics](docs/src/images/cloud-scanner-dashboard-clear.png "A example dashboard rendering cloud scanner metrics")

Principle:

![System in context diagram of cloud scanner](docs/src/images/cloud-scanner-system-in-context.png "System in context diagram of cloud scanner")

## Usage and documentation

The complete documentation: [Introduction - Boavizta cloud scanner 📡](https://boavizta.github.io/cloud-scanner/).

## Getting started 🚀

Show impacts of your EC2 instances for 10 hours of use.

```sh
export AWS_PROFILE='<YOUR_PROFILE_NAME>'

# Get impacts of 10 hours of use (on your default account region)
cargo run standard --hours-use-time 10 | jq

# Same thing but as metrics
cargo run  -- --as-metrics standard --hours-use-time 10

# Same query for explicit region
cargo run  -- --aws-region eu-west-3 standard --hours-use-time 10 | jq
```

## Usage as CLI 💻

### Using  public docker image 🐳

```sh
docker pull ghcr.io/boavizta/cloud-scanner-cli:latest
docker run -it ghcr.io/boavizta/cloud-scanner-cli:latest --help

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

See [Run as docker - Boavizta cloud scanner 📡](https://boavizta.github.io/cloud-scanner/how-to/docker-guide.html)

### Building local executable 🦀

```sh
cargo build --release
```

See [Building CLI - Boavizta cloud scanner 📡](https://boavizta.github.io/cloud-scanner/how-to/building-cli.html)

### CLI options

```sh


List aws instances and their environmental impact (from Boavizta API)

Usage: cloud-scanner-cli [OPTIONS] <COMMAND>

Commands:
  standard        Get Average (standard) impacts for a given usage duration (without considering cpu use)
  measured        Get impacts related to instances usage rate (take into account instance cpu  use)
  list-instances  Just list instances and their metadata (without impacts)
  help            Print this message or the help of the given subcommand(s)

Options:
  -a, --aws-region <AWS_REGION>
          AWS region (The default aws profile region is used if not provided)
  -b, --boavizta-api-url <BOAVIZTA_API_URL>
          Optional Boavizta API URL if you want to use your own instance (URL without the trailing slash, e.g. https://api.boavizta.org)
  -t, --filter-tags <FILTER_TAGS>
          Filter instances on tags (like tag-key-1=val_1 tag-key_2=val2)
  -v, --verbosity...
          Enable logging, use multiple `v`s to increase verbosity
  -m, --as-metrics
          Returns OpenMetrics (Prometheus) instead of json output
  -h, --help
          Print help information
  -V, --version
          Print version information
```

See [CLI options - Boavizta cloud scanner 📡](https://boavizta.github.io/cloud-scanner/reference/cli-options.html)

### Passing AWS credentials

Easiest way to pass aws credential is use an environment variable to use a specific aws profile.

```sh
export AWS_PROFILE='<YOUR_PROFILE_NAME>'
```

See [AWS authentication - Boavizta cloud scanner 📡](https://boavizta.github.io/cloud-scanner/how-to/passing-aws-credentials.html)

## Usage as a serverless app (aws lambda) ⚡

The serverless application for aws is deployed with the serverless framework.
It creates a role configured to get sufficient permission to scan your resources without requesting authentication.

- [Quickstart as serverless ⚡ - Boavizta cloud scanner 📡](https://boavizta.github.io/cloud-scanner/tutorials/quickstart-serverless.html)
- [Serverless design - Boavizta cloud scanner 📡](https://boavizta.github.io/cloud-scanner/reference/serverless-design.html)

### Deploy the app

```sh
npm i
export aws_profile = <my profile>
serverless deploy
```

## Output formats

Cloud scanner CLI and serverless application return data as _json_ or _Open Metrics_ (Prometheus) format.

See [Output data - Boavizta cloud scanner 📡](https://boavizta.github.io/cloud-scanner/reference/output-data.html)

## ⚠ Current limitations

Cloud scanner is stable, but with limited functionality.

At the moment:

- Returns _empty_ impacts (i.e. zero values) for EC2 the instance _types_ that are not listed in Boavizta database.
- `--aws-region` flag only supports eu-based aws region (eu-east-1,eu-central-1,eu-north-1,eu-south-1,eu-west-1,eu-west-2,eu-west-3).
- Returns _default_ impacts of AWS instances. It does not yet analyses instance usage (cpu workload) to calculate the impacts, but rather returns the _default_ impact data provided by Boavizta API for each instance type for a given use duration. (i.e. using instance CPU load through the `measured` command line flag has no effect).
- Filtering instances by tag is not yet supported.

This is work in progress, and development version may already implement theses functionalities. So have a look at the [changelog](https://github.com/Boavizta/cloud-scanner/blob/main/CHANGELOG.md) and [Issues · Boavizta/cloud-scanner](https://github.com/Boavizta/cloud-scanner/issues) on this repository.
