# Quick start - Run CLI using Cargo 🦀

This quick start will show the impacts of your EC2 instances for 10 hours of use, by building a local version of the cloud-scanner CLI from source.

## Prerequisites

- a Rust toolchain installed locally [rustup.rs - The Rust toolchain installer](https://rustup.rs/)
- A working AWS account (and you AWS CLI profile already configured)
- optional: [jq](https://stedolan.github.io/jq/) to format json results

## Compile an run CLI 💻

1. Clone the cloud-scanner repository
2. build and run CLI

```sh
# Ensure you have a working AWS profile setup locally (like you would do for AWS CLI)
export AWS_PROFILE='<YOUR_PROFILE_NAME>'

# Get impacts of 10 hours of use (on your default account region)
cargo run estimate --use-duration-hours 10 | jq

# Get impacts of 10 hours of use, for a explicit region. Results as JSON 
cargo run -- --aws-region eu-west-1 estimate --use-duration-hours 10 | jq

# Get impacts of 10 hours of use, for a explicit region. Results as Prometheus metrics 
cargo run -- --aws-region eu-west-1 metrics --use-duration-hours 10
```
