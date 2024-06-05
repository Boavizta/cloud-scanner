# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## Unreleased

_This paragraph may describe WIP/unreleased features. They are merged to main branch but not tagged._

## [3.0.0]-2024-06-05

## What's Changed

- [352 estimate impacts of an existing inventory by demeringo · Pull Request #505 · Boavizta/cloud-scanner](https://github.com/Boavizta/cloud-scanner/pull/505). ⚠ This introduces breaking changes on the CLI options. The option to get results as metrics (using the flag `--as-metrics` on the 'estimate' command is replaced by a direct command name `metrics`).
- [JSON output: use snake_case for all keys. · Issue #521 · Boavizta/cloud-scanner](https://github.com/Boavizta/cloud-scanner/issues/521)

## Added

- [Add metadata to the inventory · Issue #508 · Boavizta/cloud-scanner](https://github.com/Boavizta/cloud-scanner/issues/508)

## [2.0.5]-2024-04-12

## Added

- 406 expose additional metrics like CPU usage and storage size by @demeringo in https://github.com/Boavizta/cloud-scanner/pull/464
- Add -summary option to estimate command  by @jnioche in https://github.com/Boavizta/cloud-scanner/pull/466

## What's Changed

- Organises the dashboard into rows: Intro / PE / GWP / ADP by @jnioche in https://github.com/Boavizta/cloud-scanner/pull/457
- chore: dependencies by @demeringo in https://github.com/Boavizta/cloud-scanner/pull/459
- 460 update serverless framework dependencies by @demeringo in https://github.com/Boavizta/cloud-scanner/pull/461
- Remove old dashboard from the docker-compose example, fixes #438 by @jnioche in https://github.com/Boavizta/cloud-scanner/pull/449
- chore: dependencies updates by @demeringo in https://github.com/Boavizta/cloud-scanner/pull/463
- test: adapt tests values to BoaviztAPI v1.2.4 by @demeringo in https://github.com/Boavizta/cloud-scanner/pull/468
- Update image version for Boavizta API in docker-compose by @jnioche in https://github.com/Boavizta/cloud-scanner/pull/470
- Update image version for Boavizta API to 1.2.4 in docker-compose by @demeringo in https://github.com/Boavizta/cloud-scanner/pull/472
- 474-High Security issues status "Unapproved" in latest alpine docker image by @damienfernandes in https://github.com/Boavizta/cloud-scanner/pull/475
- Explicit versions of images in docker compose by @jnioche in https://github.com/Boavizta/cloud-scanner/pull/481
- Missing param summary_only in cloud_scanner_lambda, fixes #473 by @jnioche in https://github.com/Boavizta/cloud-scanner/pull/478
- New version of dashboard by @jnioche in https://github.com/Boavizta/cloud-scanner/pull/480

## New Contributors

- @jnioche made their first contribution in https://github.com/Boavizta/cloud-scanner/pull/457

**Full Changelog**: https://github.com/Boavizta/cloud-scanner/compare/v2.0.4...v2.0.5


## [2.0.4]-2024-03-01

### Added

- Add a Prometheus data volume in the docker-compose example [Document how to persist prometheus data when the container is recreated · Issue #434 · Boavizta/cloud-scanner](https://github.com/Boavizta/cloud-scanner/issues/434)
- Support all AWS regions [Improve region support · Issue #48 · Boavizta/cloud-scanner](https://github.com/Boavizta/cloud-scanner/issues/48)

### Breaking change

- Cli and Serverless parameters `--hours-use-time` were renamed into `--use-duration-hours`. Short form remain `-u`.

### Changed

- [Refactor code to make it more readable · Issue #209 · Boavizta/cloud-scanner](https://github.com/Boavizta/cloud-scanner/issues/209)
- [Improve error message when a region is incorrect · Issue #439 · Boavizta/cloud-scanner](https://github.com/Boavizta/cloud-scanner/issues/439)
- [Release 2.0.3 uses dev Boavizta API URL instead of stable · Issue #425 · Boavizta/cloud-scanner](https://github.com/Boavizta/cloud-scanner/issues/425)
- [Improve doc about passing AWS credentials · Issue #77 · Boavizta/cloud-scanner](https://github.com/Boavizta/cloud-scanner/issues/77)

### New contributors 

- Thanks to @jnioche for his contribution to testing and documenting issues related to regions !

## [2.0.3]-2024-01-17

- [Use Boavizta API v1.2.0 · Issue #416 · Boavizta/cloud-scanner](https://github.com/Boavizta/cloud-scanner/issues/416)

## [2.0.2]-2024-01-17

### Added

- [Revamp demo grafana dashboard to display individual resource metrics · Issue #403 · Boavizta/cloud-scanner](https://github.com/Boavizta/cloud-scanner/issues/403)

### Changed

- Use latest released version of Rust client for Boavizta API v1.0.1

## [2.0.1]-2024-01-17

### Added

- [Expose resource tags in metrics labels · Issue #407 · Boavizta/cloud-scanner](https://github.com/Boavizta/cloud-scanner/issues/407)

### Changed

- Make _filter tags_ optional in the API routes.
- [Update to latest aws SDK 1.x · Issue #410 · Boavizta/cloud-scanner](https://github.com/Boavizta/cloud-scanner/issues/410)

## [2.0.0-alpha]-2024-01-10

### Added

- Return instance state (either *Running* or *Stopped*) with the inventory: [Add instance state to the inventory](https://github.com/Boavizta/cloud-scanner/issues/396).
- Return metrics of individual resources: [Expose individual metrics (label metrics with resource id's and other metadata)](https://github.com/Boavizta/cloud-scanner/issues/379)

### Changed

- **Breaking change**: Renamed the count summary metrics (_instances_ become _resources_ because we now take into account additional resources like storage):
  - `boavizta_number_of_instances_total` becomes `boavizta_number_of_resources_total`
  - `boavizta_number_of_instances_assessed` becomes `boavizta_number_of_resources_assessed`
- Use Boavizta API v1.1.0 in docker-compose (this adds support for additional instances): https://github.com/Boavizta/cloud-scanner/issues/386
- Update logo in documentation: https://github.com/Boavizta/cloud-scanner/pull/381
- Add link checker when publishing documentation: https://github.com/Boavizta/cloud-scanner/pull/382
- Add logo in the doc website: https://github.com/Boavizta/cloud-scanner/pull/383

## [1.0.1]-2023-10-28

### Added

- [Return ids of the instances attached to a volume when doing block storage inventory · Issue #366 · Boavizta/cloud-scanner](https://github.com/Boavizta/cloud-scanner/issues/366)

## [1.0.0]-2023-10-12

First stable release of cloud-scanner that supports latest Boavizta API v1.x [Releases · Boavizta/boaviztapi](https://github.com/Boavizta/boaviztapi/releases).

_Cloud-scanner 1.x is really based on the previous releases 0.4.x and 0.3.x , but renamed to v1.0.x to follow the Boavizta API naming._

Thanks to the contributions of @damienfernandes, @notCamelCaseName and the great work of members of the Boavizta collective, with a special mention for @da-ekchajzer !

## [0.4.1]-2023-10-11

### Changed

- Update the parsing of JSON returned by Boavizta API to follow latest changes in API v1.0.0 (see [Align verbose and non verbose json output format · Issue #229 · Boavizta/boaviztapi](https://github.com/Boavizta/boaviztapi/issues/229)).

## [0.4.0]-2023-10-09

### Added

- 🧪 Experimental feature: [Provide estimations related to storage · Issue #272 · Boavizta/cloud-scanner](https://github.com/Boavizta/cloud-scanner/issues/272). Use the `--include-block-storage` command line flag or parameter to consider block storage (either in inventory or when requesting an estimation of impacts.). This parameter defaults to `false` . This means that by default block storage (volumes) are not counted in the inventory nor in the results.

⚠ In any case, for storage, the impacts of the _use_ phase are _not_ counted. Boavizta API returns only the impacts of the _manufacturing_ phase for HDD and SSD.

```sh
# Experimental: get impacts of instances and attached storage
cargo run estimate --hours-use-time 1 --include-block-storage --output-verbose-json
```

## [0.3.0-alpha4]-2023-09-20

### Added

- [Add optional verbose output with the details of the calculation  · Issue #333 · Boavizta/cloud-scanner](https://github.com/Boavizta/cloud-scanner/issues/333). This introduces and additional (optional CLI option). Verbose is deactivated by default.
- [Add a parameter to pass the duration of use when exposing metrics and data in standalone server mode · Issue #332 · Boavizta/cloud-scanner](https://github.com/Boavizta/cloud-scanner/issues/332), thanks to a contribution from @damienfernandes.

### Changed

- Format of json output evolved: It now contains a additional field `raw_data` that returns the json data exactly as fetched from Boavizta API (so the format varies depending if using verbose output or not).
- dependencies updates
- doc update

## [0.3.0-alpha3]-2023-07-27

### Added

- [Log time spend on actions · Issue #289 · Boavizta/cloud-scanner](https://github.com/Boavizta/cloud-scanner/issues/289)
- [Return inventory as json in the standalone server mode · Issue #286 · Boavizta/cloud-scanner](https://github.com/Boavizta/cloud-scanner/issues/286) thanks to a contribution of @notCamelCaseName
- [Return impacts as json in the standalone server mode · Issue #287 · Boavizta/cloud-scanner](https://github.com/Boavizta/cloud-scanner/issues/287)
- [Expose OpenAPI spec · Issue #285 · Boavizta/cloud-scanner](https://github.com/Boavizta/cloud-scanner/issues/285)

### Changed

- [Update to latest AWS SDK · Issue #308 · Boavizta/cloud-scanner](https://github.com/Boavizta/cloud-scanner/issues/308)
- Serverless: update `lambda_runtime` and `lambda_http` crates to v 0.8.1, bump version of cloud_scanner_lambda package to `0.3.0-alpha3`.

## [0.3.0-alpha2]-2023-06-26

Minor adaptations to the not yet released Boavizta API V1.0alpha.

## [0.3.0-alpha]-2023-05-16

This alpha release contains changes to integrate with the new version of Boavizta API (v0.3.0).

It does not bring any new feature but allow retrieving the latest data set.

## [0.2.4]-2023-04-19

### Changed

- Use new Boavizta API SDK for API v0.2.2 and use new API routes (following deprecation of AWS specific URLs). [Upgrade  to stable Boavizta API v0.2.x SDK · Issue #243 · Boavizta/cloud-scanner](https://github.com/Boavizta/cloud-scanner/issues/243) and [Update SDK for boaviztapi v2.x · Issue #4 · Boavizta/boaviztapi-sdk-rust](https://github.com/Boavizta/boaviztapi-sdk-rust/issues/4)
- [Use Boavizta API v0.2.2 in the example docker-compose · Issue #208 · Boavizta/cloud-scanner](https://github.com/Boavizta/cloud-scanner/issues/208))
- Doc: add aws command to list/start/stop instances in the _testing_ chapter of the doc.

## [0.2.3]-2023-03-13 - Douala release 🌴

### Changed

- Fix: use duration metric does not return zero anymore [Value returned for boavizta_duration_of_use_hours metric is always zero · Issue #206 · Boavizta/cloud-scanner](https://github.com/Boavizta/cloud-scanner/issues/206)
- Docs: [Add link and icon to github repo in the published documentation  · Issue #223 · Boavizta/cloud-scanner](https://github.com/Boavizta/cloud-scanner/issues/223) + several schematics
- Docs: [Add Link to boavizta cloud impact  · Issue #222 · Boavizta/cloud-scanner](https://github.com/Boavizta/cloud-scanner/issues/222)
- Dependencies updates and use Rust 1.68.

## [0.2.2]-2023-01-07

### Changed

- Support all US aws regions.

## [0.2.1]-2023-01-05

### Added

- Filter instances on tags with serveless / lambda functions.
- Doc update

## [0.2.1-alpha.2]-2022-12-29

### Added

- Filter instances on tags

### Changed

- Update dependencies

## [0.2.1-alpha1]-2022-12-22

### Added

- Retrieve instance tags when doing resources inventory. See https://github.com/Boavizta/cloud-scanner/issues/189.

## [0.2.0]-2022-12-16

### Changed

- Clean up code to get rid of clippy warnings
- Update dependencies
- Use publicly published Boavizta API v0.2.x in tests.
- Doc updates.

## [0.2.0-alpha.1]-2022-12-07

### Changed

- Use Boavizta API v0.2.x.
- Take in consideration the instances workload (cpu load) to calculate the impacts.

## [0.1.1]- 2022-12-07

### Added

- Display scanner version on metrics server status route https://github.com/Boavizta/cloud-scanner/issues/179

## [0.1.0-alpha.2]- 2022-12-07

### Changed

- Pin docker image versions in docker compose, see https://github.com/Boavizta/cloud-scanner/issues/175

## [0.1.0-alpha.1]- 2022-11-26

### Changed

- Major refactoring to ease future evolution and testing.
- Simplified CLI commands.

## [0.0.9]- 2022-11-18

### Added

- An example of usage in a monitoring stack (including dashboard) of via docker-compose. See [Quickstart dashboard](https://boavizta.github.io/cloud-scanner/tutorials/quickstart-dashoard-docke.html) in documentation. Many Thanks to @obinjf for his contributions.

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
