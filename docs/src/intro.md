# About cloud-scanner  📡

[Boavizta Cloud-scanner](https://github.com/Boavizta/cloud-scanner) returns environmental impacts of your AWS Instances (EC2) usage.

It combines real time _inventory_ and _usage_ data from your AWS account with [Boavizta API](https://github.com/Boavizta/boaviztapi/) to offer a  view of your impacts on a given region.

- multi criteria: Primary Energy consumption (PE), Abiotic resource depletion potential (ADP), and Global Warming Potential (GWP)
- multi stage: separate impacts of ressources Usage and Manufacture

Estimations can be filtered by tags. It eases attribution to a specific server, environment, application or service.

Cloud-scanner can be used:

- from command line, to get an immediate view of your impacts 💻
- as a metric server . You can use it to monitor and display real time impacts in a dashboard 📊

![A example dashboard rendering cloud scanner metrics](images/cloud-scanner-dashboard-clear.png "A example dashboard rendering cloud scanner metrics")

Cloud-scanner is an Open Source application maintained here: <https://github.com/Boavizta/cloud-scanner>.

## How it works

![System in context diagram of cloud scanner](images/cloud-scanner-system-in-context.png "System in context diagram of cloud scanner")

## ⚠ Alpha version

Cloud scanner is stable, but with limited functionality.

This is work in progress, and development versions (`dev` branch of the repository) may already implement new functionalities. So have a look at the [changelog](https://github.com/Boavizta/cloud-scanner/blob/main/CHANGELOG.md) and [Issues · Boavizta/cloud-scanner](https://github.com/Boavizta/cloud-scanner/issues) on this repository.
