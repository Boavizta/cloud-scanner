# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Use the instances workload (cpu load) to tune the results.

## Unrealeased [0.1.0-alpha.1]- 2022-11-24

### Changed

- Major refactoring to ease future evolution and testing.
- Simplified CLI commands.

## [0.0.9]- 2022-11-18

### Added

- A example of usage in a monitoring stack (including dashboard) of via docker-compose. See [Quickstart dashboard](https://boavizta.github.io/cloud-scanner/tutorials/quickstart-dashoard-docke.html) in documentation. Many Thanks to @obinjf for his contributions.

## [0.0.8] - 2022-11-13

### Added

- Option to returns metrics through an http endpoint (Start the CLI with `cloud-scanner serve`). Metrics are recalculated each time the endpoint is scraped for example at <http://localhost:8000/metrics?aws_region=eu-west-1>).  
⚠ Running metric server in container require setting  extra variables:
  - to map AWS credentials 
  - to map SSL ca certificates 
  - and more importantly to configure rocket to listen to 0.0.0.0 instead of 127.0.0.1 (which is internal to the container): `ROCKET_ADDRESS=0.0.0.0`
``` sh
docker run -it -p 8000:8000 -v /etc/ssl/certs/ca-certificates.crt:/etc/ssl/certs/ca-certificates.crt -v $HOME/.aws/credentials:/root/.aws/credentials:ro -e ROCKET_ADDRESS=0.0.0.0 -e ROCKET_PORT=8000 -e AWS_PROFILE=$AWS_PROFILE ghcr.io/boavizta/cloud-scanner-cli:latest serve
```

## [0.0.7] - 2022-10-06

### Changed

- Update dependencies.
- Use feature flag on lambda http (support alb and apigw_rest).
- Clean up code to get rid of Clippy warnings.
- Improve error handling using Anyhow (see <https://github.com/Boavizta/cloud-scanner/issues/17>)
- Upgrade to Clap v 4.0.x to provide CLI parsing and help.
- Fix wrong default API url in CLI and serverless environment, see <https://github.com/Boavizta/cloud-scanner/issues/125>
- Use a public crate to provide boavizta-api-sdk . Previous version relies on local SDK code in this repository. This SDK is now maintained in its own repository at <https://github.com/Boavizta/boaviztapi-sdk-rust> (see <https://github.com/Boavizta/cloud-scanner/issues/112>).
- Cargo run CLI by default (see <https://github.com/Boavizta/cloud-scanner/issues/123>)

## [0.0.6] - 2022-09-15

### Added

- Support using a custom (private) Boaviztapi URL instead of public demo instance.
  
## [0.0.5] - 2022-08-23

### Added

- Initiate documentation as mdBook (see <https://github.com/Boavizta/cloud-scanner/issues/61>)
- Publish doc in CI [Introduction - Boavizta cloud scanner 📡](https://boavizta.github.io/cloud-scanner/)
- Support scanning a region different from where the lambda is deployed.

### Changed

-Update dependencies (most notably AWS sdk 0.17).

## [0.0.4] - 2022-06-24

### Added

- Serverless app supports `use hours_use_time` in default scan
  - passed as request parameters (example query `https://xxxxx.execute-api.eu-west-1.amazonaws.com/dev/scan?hours_use_time=10`
  - this parameter is mandatory
- Update serverless app to use `aws_region` in default scan
  - ⚠ This _optional_ parameter is not yet supported by the scanner lib (which always defaults to the default region of the lambda)
- Export scan result summary as OpenMetrics/Prometheus metrics (see `-m` or `--as-metrics` flag in CLI).
- Add a route in serverless app that returns metrics for one hour use (e.g. `https://xxxx/dev/metrics?aws_region=eu-west-3`)

### Changed

- Update AWS sdk dependencies.
- Remove the out-file CLI option

## [0.0.3] - 2022-06-10

### Changed

- Reduce size of docker image (use Alpine (5MB) instead of Ubuntu (80MB))
- Fix github workflow to publish image only on merge to main branch
- Serverless scanner: display version of library in log

## [0.0.2] - 2022-06-10

### Added

- Allow deployment as serverless application (⚠ but with very limited functionality, only default scan works for now).

## [0.0.1] - 2022-05-06

- First alpha version.
