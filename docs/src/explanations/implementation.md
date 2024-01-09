# Internal implementation

Cloud scanner is mainly written in Rust and code is organized in 2 main modules;

- cloud-scanner-cli: the main codebase of cloud scanner
  - model of cloud resource
  - cloud_inventory / aws_inventory: functions to perform inventory of cloud resources
  - impact provider (module responsible for getting impacts of from Boavizta API)
  - metrics exporter
  - Command Line Interface (CLI) functionality
  - Server part (exposing metrics or json output)
- cloud-scanner-lambda
  - A small wrapper around the cloud-scanner cli to ease deployment as AWS lambda
