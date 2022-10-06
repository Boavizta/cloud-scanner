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
cargo run standard --hours-use-time 10 | jq

# Same thing but as metrics
cargo run -- --as-metrics standard --hours-use-time 10

# Same query for explicit region
cargo run -- --aws-region eu-west-3 standard --hours-use-time 10 | jq
```
