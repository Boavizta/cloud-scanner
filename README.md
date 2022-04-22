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

Not yet implemented

### Run

```sh
export AWS_PROFILE='<YOUR_PROFILE_NAME>'
cd cloud-scanner-cli
cargo run -- --text
```

## Output format

Cloud scanner prints some instance metadata (instanceid, tags and type) on stdout.

Will soon return

- an array of instances (json)
- an array of instances with their usage/workload (json)
- return an array of instances with their impact as json (Boavizta output, see [docs/sample_result.json](docs/sample_result.json))
