# cloud-scanner

Collect aws cloud usage data, so that it can be combined with impact data of Boavizta API.

It can be used from command line app (CLI) or be deployed as a serverless application and return metrics (that can be pushed to a dashboard).

⚠ Work in progress ! See the [changelog](CHANGELOG.md).

At the moment it just returns _standard_ impacts of aws instances in the default region of your account. It does not use metrics of instance usage to calculate the impacts, but rather returns the _default_ impact data provided by Boavizta API for each instance type for a given use duration.

![Scanner in context](docs/out/../../out/docs/cloud-scanner-system-in-context/cloud-scanner-system-in-context.png)

## Getting started 💻

### List standard impacts of AWS instances (for 10 hours of use)

Using default account region.

```sh
export AWS_PROFILE='<YOUR_PROFILE_NAME>'

# Estimate impact for 10 hours of use
cargo run --bin cloud-scanner-cli standard --hours-use-time 10 | jq
```

## Usage as CLI 💻

### Run public docker image

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

### Build a local docker image

```sh
# Local build of docker image
docker build . --tag cloud-scanner-cli
# Test run
docker run -it cloud-scanner-cli --help
```

### Building local executable

```sh
cargo build --release
```

### Cli options

```sh
cloud-scanner-cli 0.0.4
Olivier de Meringo <demeringo@gmail.com>
List aws instances and their environmental impact (from Boavizta API)

USAGE:
    cloud-scanner-cli [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -a, --aws-region <AWS_REGION>
            AWS region (default profile region is assumed if not provided)

    -b, --boavizta-api-url <BOAVIZTA_API_URL>
            Optional Boavizta API URL (if you want to use your own instance)

    -h, --help
            Print help information

    -m, --as-metrics
            Returns OpenMetrics (Prometheus like) instead of json output

    -t, --filter-tags <FILTER_TAGS>
            Filter instances on tags (like tag-key-1=val_1 tag-key_2=val2)

    -v, --verbosity
            Enable logging, use multiple `v`s to increase verbosity

    -V, --version
            Print version information

SUBCOMMANDS:
    help              Print this message or the help of the given subcommand(s)
    list-instances    just list instances and their metadata (without impacts)
    measured          get impacts related to measured instance usage: depending on usage rate
                          (use instance workload),
    standard          get Average (standard) impacts for a given usage duration
```

### Get measured impacts of instances for a given period

This uses the workload measured on instances to provide more realistic impacts.

⚠ TODO

- pass period parameter (start date / end date)
- define a sampling rate for cloudwatch metrics retrieval?

### Passing AWS credentials

Easiest way to pass aws credential is use an environment variable to use a specific aws profile.

```sh
export AWS_PROFILE='<YOUR_PROFILE_NAME>'
```


## Usage as a serverless app (aws lambda) ⚡

The serverless application for aws is configured to be deployed with the serverless framework.
It is configured to get sufficient permission to scan your resources without requesting authentication.

### Deploy the app

```sh
npm i
export aws_profile = <my profile>
serverless deploy
```

### Serverless routes

#### Scan account / region

Returns results in json format (see below, same as CLI)

https://xxxxx.execute-api.eu-west-1.amazonaws.com/dev/scan?hours_use_time=5&aws_region=eu-west-1

Use `hours_use_time` and `aws_region` parameters in the query

#### Get Metrics

https://xxxxx.execute-api.eu-west-1.amazonaws.com/dev/metrics?aws_region=eu-central-1

returns metrics for one hour of use in prometheus format.
Use `aws_region` parameters in the query.

```
# HELP boavizta_number_of_instances_total Number of instances detected during the scan.
# TYPE boavizta_number_of_instances_total gauge
boavizta_number_of_instances_total{awsregion="eu-central-1",country="DEU"} 7
# HELP boavizta_number_of_instances_assessed Number of instances that were considered in the measure.
# TYPE boavizta_number_of_instances_assessed gauge
boavizta_number_of_instances_assessed{awsregion="eu-central-1",country="DEU"} 5
# HELP boavizta_duration_of_use_hours Number of instances detected during the scan.
# TYPE boavizta_duration_of_use_hours gauge
boavizta_duration_of_use_hours{awsregion="eu-central-1",country="DEU"} 1.0
# HELP boavizta_pe_manufacture_megajoules Power consumed for manufacture.
# TYPE boavizta_pe_manufacture_megajoules gauge
boavizta_pe_manufacture_megajoules{awsregion="eu-central-1",country="DEU"} 1760.0
# HELP boavizta_pe_use_megajoules Power consumed during usage.
# TYPE boavizta_pe_use_megajoules gauge
boavizta_pe_use_megajoules{awsregion="eu-central-1",country="DEU"} 0.86
# EOF
```

## Output formats

### JSON output (the default)

Cloud scanner returns a json array of instances metadata (instance*id, type usage_data and and usage impacts) on \_stdout*.

⚠ Returns _empty_ impacts when the _instance type_ is not known in Boavizta database

```json
[
  {
    "instance_id": "i-001dc0ebbf9cb25c0",
    "instance_type": "t2.micro",
    "usage_data": {
      "hours_use_time": 5,
      "usage_location": "IRL"
    },
    "impacts": {}
  },
  {
    "instance_id": "i-004599844f7c24814",
    "instance_type": "t2.small",
    "usage_data": {
      "hours_use_time": 5,
      "usage_location": "IRL"
    },
    "impacts": {}
  },
  {
    "instance_id": "i-075444d7293d8bd76",
    "instance_type": "t2.micro",
    "usage_data": {
      "hours_use_time": 5,
      "usage_location": "IRL"
    },
    "impacts": {}
  },
  {
    "instance_id": "i-033df52f12f30ca66",
    "instance_type": "m6g.xlarge",
    "usage_data": {
      "hours_use_time": 5,
      "usage_location": "IRL"
    },
    "impacts": {
      "adp": {
        "manufacture": 0.0084,
        "unit": "kgSbeq",
        "use": 1.7e-9
      },
      "gwp": {
        "manufacture": 87,
        "unit": "kgCO2eq",
        "use": 0.029
      },
      "pe": {
        "manufacture": 1100,
        "unit": "MJ",
        "use": 0.82
      }
    }
  }
]
```

### OpenMetrics/Prometheus output

If using `--as-metrics` or `-m` option, cloud-scanner returns consolidated results as OpenMetric/Prometheus format insted of json details.

When using the metric output format, you cannot see the individual impacts of each instance. Instead, impacts of all instances are added to provide a global figure.

```
cargo run --bin cloud-scanner-cli -- --as-metrics  standard -u 1

# HELP boavizta_number_of_instances_total Number of instances detected during the scan.
# TYPE boavizta_number_of_instances_total gauge
boavizta_number_of_instances_total{awsregion="eu-west-1",country="IRL"} 9
# HELP boavizta_number_of_instances_assessed Number of instances that were considered in the measure.
# TYPE boavizta_number_of_instances_assessed gauge
boavizta_number_of_instances_assessed{awsregion="eu-west-1",country="IRL"} 6
# HELP boavizta_duration_of_use_hours Number of instances detected during the scan.
# TYPE boavizta_duration_of_use_hours gauge
boavizta_duration_of_use_hours{awsregion="eu-west-1",country="IRL"} 1.0
# HELP boavizta_pe_manufacture_megajoules Power consumed for manufacture.
# TYPE boavizta_pe_manufacture_megajoules gauge
boavizta_pe_manufacture_megajoules{awsregion="eu-west-1",country="IRL"} 2060.0
# HELP boavizta_pe_use_megajoules Power consumed during usage.
# TYPE boavizta_pe_use_megajoules gauge
boavizta_pe_use_megajoules{awsregion="eu-west-1",country="IRL"} 0.228
# EOF
```

## ⚠ Current limitations

- Return empty impacts when the instance _type_ is not listed in Boavizta database.
- `--aws-region` flag only supports eu-based aws regions for the time being (eu-east-1,eu-central-1,eu-north-1,eu-south-1,eu-west-1,eu-west-2,eu-west-3)
- Always returns _standard_ impacts: using instance workload to assess impact is not yet implemented (i.e. using CPU load through the `measured` command has no effect yet).
- Filtering instances by tag is not yet supported.
- Passing a private Boavizta API URL is not yet implemented

### Generate / update Boavizta API sdk

```sh
docker run --rm -v "${PWD}:/local" openapitools/openapi-generator-cli generate -i http://api.boavizta.org/openapi.json   -g rust  -o /local/boavizta-api-sdk --package-name boavizta_api_sdk
```
