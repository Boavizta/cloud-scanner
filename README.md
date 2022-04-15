# cloud-scanner

Collect cloud usage data, so that it can be combined with impact data of Boavizta API.

⚠ Very early Work in progress !

At the moment it just list instances of your default region, without any usage nor impact data.

![Scanner in context](docs/out/../../out/docs/cloud-scanner-system-in-context/cloud-scanner-system-in-context.png)

## Usage

### Passing credentials

Easiest way is use an environment variable to use a specific aws profile.

```sh
export AWS_PROFILE='<YOUR_PROFILE_NAME>'
```

### Filtering resources

TODO

### Run

```sh
export AWS_PROFILE='<YOUR_PROFILE_NAME>'
cd cloud-scanner-cli
cargo run
```

## Output format

Cloud scanner returns an array of instances with their impacts in json format.

See [docs/sample_result.json](docs/sample_result.json)
