# About cloud-scanner  📡

Cloud-scanner returns environmental impacts of your AWS Instances usage.

It combines real time usage data from your AWS account with [Boavizta API](https://github.com/Boavizta/boaviztapi/) to offer a global view of your impacts on a given region.

Cloud-scanner can be used:

- from command line 💻
- as a serverless application deployed with AWS lambda ⚡

Cloud-scanner can be automated to produce metrics at regular interval and monitor your impacts in a dashboard.

![cloud-scanner-metrics-dashboard](images/cloud-scanner-dashboard-clear.png)

![Scanner in context](images/cloud-scanner-system-in-context.png)

## ⚠ Alpha version

Cloud scanner is stable, but with limited functionality.

At the moment it only returns _default_ impacts of AWS instances. It does not yet analyses instance usage (workload) to calculate the impacts, but rather returns the _default_ impact data provided by Boavizta API for each instance type for a fixed duration of use.

This is work in progress, and development version may already implement theses functionalities. So have a look at the [changelog](CHANGELOG.md) and [Issues · Boavizta/cloud-scanner](https://github.com/Boavizta/cloud-scanner/issues) on this repository.
