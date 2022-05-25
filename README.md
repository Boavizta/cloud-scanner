# cloud-scanner

Collect aws cloud usage data, so that it can be combined with impact data of Boavizta API.

⚠ Very early Work in progress !

At the moment it just returns impacts of your aws instances. It does not use metrics of instance usage to calculate the impacts, but rather returns the _default_ impact data provided by API for each instance type.

![Scanner in context](docs/out/../../out/docs/cloud-scanner-system-in-context/cloud-scanner-system-in-context.png)

## Getting started

### List standard impacts of AWS instances of the account

Using default account region.

```sh
export AWS_PROFILE='<YOUR_PROFILE_NAME>'

# Estimate impact for 10 hours of use 
cargo run standard --hours-use-time 10| jq
```

## Usage

### Cli options

```sh
cargo run -- --help

cloud-scanner-cli 0.0.1
List aws instances and their environmental impact (from Boavizta API)

USAGE:
    cloud-scanner-cli [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -a, --aws-region <AWS_REGION>      AWS region (default profile region is assumed if not
                                       provided) [default: ]
    -h, --help                         Print help information
    -t, --filter-tags <FILTER_TAGS>    Filter instances on tags (like tag-key-1=val_1
                                       tag-key_2=val2)
    -V, --version                      Print version information

SUBCOMMANDS:
    help              Print this message or the help of the given subcommand(s)
    list-instances    just list instances and their metadata (without impacts)
    measured          get impacts related to measured instance usage: depending on usage rate
                          (use instance workload),
    standard          get Average (standard) impacts for a given usage duration
```

### Get measured imapcts of your instances for a given period

This uses the workload measured on instances to provide more realistic impacts.

⚠ TODO

- pass period parameter (start date / end date)
- define a sampling rate for cloudwatch metrics retrieval?

### Passing AWS credentials

Easiest way to pass aws credential is use an environment variable to use a specific aws profile.

```sh
export AWS_PROFILE='<YOUR_PROFILE_NAME>'
```

## Output format

Cloud scanner returns json array of instances metadata (instance_id, tags, type and usage impacts) on stdout.

## Current limitations

- Query only the default region of you AWS profile
- Instance workload (i.e. CPU load) is not - yet - used to assess impacts.
- Filtering instances by tag is not yet supported.

## Generate / update Boavizta API sdk

```sh
docker run --rm -v "${PWD}:/local" openapitools/openapi-generator-cli generate -i http://api.boavizta.org/openapi.json   -g rust  -o /local/boavizta-api-sdk --package-name boavizta_api_sdk
```
