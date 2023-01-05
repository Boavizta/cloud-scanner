# Filter by tags

You can limit cloud-scanner results (metriccs or json) to the cloud resources that match specific tags.

You can use multiple tags. 

Works with inventory or estimates.

## Filter in cli

```sh
# Retrieve only resources with the following tags:
#  CustomTagNameForDebug=olivierdemeringoadm
#     and 
#  Name=test-boavizta
cargo run -- -t Name=test-boavizta-2 -t CreatorName=olivierdemeringoadm  inventory| jq
```

## Filter (with serverless/lambda)

Use one (or multiple) `filter_tag` parameter in the query string to restrict results.

```sh
# Retrieve only resources with the following tags:
#  CustomTagNameForDebug=olivierdemeringoadm
#     and 
#  Name=test-boavizta
https://<EDITED>.execute-api.eu-west-1.amazonaws.com/dev/scan?hours_use_time=5&aws_region=eu-west-1&filter_tag=CustomTagNameForDebug=olivierdemeringoadm&filter_tag=Name=test-boavizta
```

## Filter metrics

Use the `filter_tag` query parameter in the URL.

It can be used multiple time (only instances matching _all_ tags will be returned).

Example queries

- <http://localhost:8000/metrics?aws_region=eu-west-3&filter_tag=Name=boatest&filter_tag=OtherTag=other-value>
- <http://localhost:8000/metrics?aws_region=eu-west-3&filter_tag=Name=boatest>
- <http://localhost:8000/metrics?aws_region=eu-west-1&filter_tag=Name=test-boavizta>

## Important limitation

Suppose the following instances (and tags)
- instance1 (
    - ENV=prod
    - PURPOSE=CI
- instance2
  - ENV=prod
  - PURPOSE=service

- Filtering on "ENV=prod" will return instance1 and instance2
- Filtering on "ENV=prod" and PURPOSE=CI will return only instance2
- No filter will return _all_ instances
