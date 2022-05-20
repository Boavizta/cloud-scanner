# cloud-scanner

Collect aws cloud usage data, so that it can be combined with impact data of Boavizta API.

⚠ Very early Work in progress !

At the moment it just returns impacts of your aws instances. It does not use metrics of instance usage to calculate the impacts, but rather returns the _default_ impact data provided by API for each instance type.

![Scanner in context](docs/out/../../out/docs/cloud-scanner-system-in-context/cloud-scanner-system-in-context.png)

## Getting started

### List impacts of AWS instances of the account

Using default account region.

```sh
export AWS_PROFILE='<YOUR_PROFILE_NAME>'

# Estimate impact for 10 hours of use (-h 10)
cargo run -- --h 10 | jq
```

## Usage

### Cli options

```sh
cargo run -- --help

cloud-scanner-cli 0.0.1
List AWS instances and their impacts.

USAGE:
    cloud-scanner-cli [FLAGS] [OPTIONS] --hours-use-time <hours-use-time>

FLAGS:
        --help            Prints help information
        --text            Display results as text (instead of json)
    -u, --use-cpu-load    Take the CPU load of instances into consideration to estimate the impacts
    -V, --version         Prints version information

OPTIONS:
    -f, --filter-tags <filter-tags>...       Filter instances on tags (like tag-key-1=val_1 tag-key_2=val2)
    -h, --hours-use-time <hours-use-time>    The number of hours of usage for which we want to estimate the impacts
```

### Get impact of your instances for a given period

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
