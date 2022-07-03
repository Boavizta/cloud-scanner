# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

- Support scanning a region different from where the lambda is deployed.
- Use the instances workload (cpu) to tune the results.
- Use a published/versioned crate of boavizta-api-sdk (actual version relies on local sdk).

## [0.0.4] - 2022-06-24

### Added

- Serverless app supports `use hours_use_time` in default scan
  - passed as request parameters (example query `https://xxxxx.execute-api.eu-west-1.amazonaws.com/dev/scan?hours_use_time=10`
  - this parameter is mandatory
- Update serverless app to use `aws_region` in default scan
  - ⚠ This _optional_ parameter is not yet supported by the scanner lib (which always defaults to the default region of the lambda)
- Export scan result summary as OpenMetrics/Prometheus metrics (see `-m` or `--as-metrics` flag in CLI).

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
