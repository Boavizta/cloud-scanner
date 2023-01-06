# Boavizta cloud-scanner 📡

Cloud-scanner returns Boavizta environmental impact data of your cloud account.

It automates the inventory and usage of your ressources and combines it with the [Boavizta API](https://github.com/Boavizta/boaviztapi/).

- Cloud-scanner produces metrics. You can use it to monitor and display real time impacts in a dashboard.
- Cloud-scanner can also be used as a command line application to get an immediate view of your impacts.

![A example dashboard rendering cloud scanner metrics](docs/src/images/cloud-scanner-dashboard-clear.png "A example dashboard rendering cloud scanner metrics in Grafana")

How it works:

![System in context diagram of cloud scanner](docs/src/images/cloud-scanner-system-in-context.png "System in context diagram of cloud scanner")

## Usage and documentation

The complete documentation: [Introduction - Boavizta cloud scanner 📡](https://boavizta.github.io/cloud-scanner/).

## Getting started 🚀

- [Quickstart - dashboard using docker 🐳 - Boavizta cloud scanner 📡](https://boavizta.github.io/cloud-scanner/tutorials/quickstart-dashboard-docker.html)
- [Quickstart - using CLI docker 🐳 - Boavizta cloud scanner 📡](https://boavizta.github.io/cloud-scanner/tutorials/quickstart-docker.html)

## Deployment  as a serverless app (aws lambda) ⚡

Cloud scanner can also be deployed as a serverless application for aws.

- [Quickstart as serverless ⚡ - Boavizta cloud scanner 📡](https://boavizta.github.io/cloud-scanner/tutorials/quickstart-serverless.html)
- [Serverless design - Boavizta cloud scanner 📡](https://boavizta.github.io/cloud-scanner/reference/serverless-design.html)

## Output formats

Cloud scanner CLI and serverless application returns data as _json_ or _Open Metrics_ (Prometheus) format.

See [Output data - Boavizta cloud scanner 📡](https://boavizta.github.io/cloud-scanner/reference/output-data.html)

## ⚠  Perimeter / scope and limits

Cloud scanner provides impacts _only_ related to _compute_  resources (the Virtual Machines).

The limitations of cloud-scanner are documented in [Limitations - Boavizta cloud scanner 📡](https://boavizta.github.io/cloud-scanner/reference/limits.html)

Several significant aspectof the cloud provider are **excluded** :

- only measure _compute_ instances (VM's), considering that
- do not account for the surrounding cloud infrastructure (network, control plan)
- do not account for storage
- do not take into account the _overcommit_ (mutualization) or _overprovisionning_ that cloud provider may apply to provide the service.
- do not account managed services (like DB as a service or Containers as a service).
- do not account serverless (lambda) compute.
- supported regions (EU only for the time being)
- unsupported instance types returns zero for their impacts.
- we do not provide (yet) error margins https://github.com/Boavizta/boaviztapi/issues/147
- The manufacturing impacts are not amortized (i.e. value returned for manuffacturiing imacts does not consider usage duration).

This is work in progress, and development version may already implement theses functionalities. So have a look at the [changelog](https://github.com/Boavizta/cloud-scanner/blob/main/CHANGELOG.md) and [Issues · Boavizta/cloud-scanner](https://github.com/Boavizta/cloud-scanner/issues) on this repository or check the content of the `dev` branch.
