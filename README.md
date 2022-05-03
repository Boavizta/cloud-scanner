# cloud-scanner

Collect cloud usage data, so that it can be combined with impact data of Boavizta API.

⚠ Very early Work in progress !

At the moment it just list instances of your default region, without any usage nor impact data.

![Scanner in context](docs/out/../../out/docs/cloud-scanner-system-in-context/cloud-scanner-system-in-context.png)

## Getting started

### List AWS instances of the account

Using default account region.

```sh
export AWS_PROFILE='<YOUR_PROFILE_NAME>'
cd cloud-scanner-cli
cargo run -- --text
```

### Get impact of your instances for a given period

- pass period parameter (start date / end date)
- TODO: define a sampling rate for cloudwatch metrics  retrieval?


## Usage

### Passing AWS credentials

Easiest way is use an environment variable to use a specific aws profile.

```sh
export AWS_PROFILE='<YOUR_PROFILE_NAME>'
```

### Filtering resources

Not yet implemented

## Output format

Cloud scanner prints some instance metadata (instanceid, tags and type) on stdout.

Will soon return

- an array of instances (json)
- an array of instances with their usage/workload (json)
- return an array of instances with their impact as json (Boavizta output, see [docs/sample_result.json](docs/sample_result.json))

## Generate / update Boavizta API sdk

```sh
docker run --rm -v "${PWD}:/local" openapitools/openapi-generator-cli generate -i http://api.boavizta.org/openapi.json   -g rust  -o /local/boavizta-api-sdk --package-name boavizta_api_sdk
```
