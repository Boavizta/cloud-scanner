# Boavizta cloud-scanner 📡

Cloud-scanner returns Boavizta environmental impact data of your cloud account.

It automates the inventory and usage of your ressources and combines it with the [Boavizta API](https://github.com/Boavizta/boaviztapi/).

- Cloud-scanner produces metrics. You can use it to monitor and display real time impacts in a dashboard.
- Cloud-scanner can also be used as a command line application to get an immediate snapshot of your impacts.

![A example dashboard rendering cloud scanner metrics](docs/src/images/cloud-scanner-dashboard-clear.png "A example dashboard rendering cloud scanner metrics")

Principle:

![System in context diagram of cloud scanner](docs/src/images/cloud-scanner-system-in-context.png "System in context diagram of cloud scanner")

## Usage and documentation

The complete documentation: [Introduction - Boavizta cloud scanner 📡](https://boavizta.github.io/cloud-scanner/).

## Getting started 🚀

- [Quickstart - dashboard using docker 🐳 - Boavizta cloud scanner 📡](https://boavizta.github.io/cloud-scanner/tutorials/quickstart-dashboard-docker.html)
- [Quickstart - using CLI docker 🐳 - Boavizta cloud scanner 📡](https://boavizta.github.io/cloud-scanner/tutorials/quickstart-docker.html)


## Usage and documentation

The complete documentation: [Introduction - Boavizta cloud scanner 📡](https://boavizta.github.io/cloud-scanner/).


## Deployment  as a serverless app (aws lambda) ⚡

Cloud scanner can also be deployed as a serverless application for aws.

- [Quickstart as serverless ⚡ - Boavizta cloud scanner 📡](https://boavizta.github.io/cloud-scanner/tutorials/quickstart-serverless.html)
- [Serverless design - Boavizta cloud scanner 📡](https://boavizta.github.io/cloud-scanner/reference/serverless-design.html)
 
## Output formats

Cloud scanner CLI and serverless application returns data as _json_ or _Open Metrics_ (Prometheus) format.

See [Output data - Boavizta cloud scanner 📡](https://boavizta.github.io/cloud-scanner/reference/output-data.html)

## ⚠ Current limitations

Cloud scanner is stable, but with limited functionality.

- Only EU region are supported: `--aws-region` flag only supports eu-based aws regions for the time being (eu-east-1,eu-central-1,eu-north-1,eu-south-1,eu-west-1,eu-west-2,eu-west-3)
- Cloud-scanner return empty impacts if the instance _type_ is not listed in Boavizta database.
- Filtering instances by tag is not yet supported.

This is work in progress, and development version may already implement theses functionalities. So have a look at the [changelog](https://github.com/Boavizta/cloud-scanner/blob/main/CHANGELOG.md) and [Issues · Boavizta/cloud-scanner](https://github.com/Boavizta/cloud-scanner/issues) on this repository or check the content of the `dev` branch.
