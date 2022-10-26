# Setup monitoring dashboard

Cloud-scanner can be used to export metrics related to the impact of your cloud account.

The metrics are exposed in Prometheus/OpenMetrics format. You can use it to feed a monitoring dashboard and keep an eye on the evolution of impacts of your usage.

![cloud-scanner-metrics-dashboard](../images/cloud-scanner-dashboard-clear.png)

## Overview

1. Deploy cloud-scanner as a serverless app in your AWS account 
2. Setup Prometheus to scrape the metrics
3. Configure a grafana dashboard to display the results.

## Detailed steps


It is often easier (and safer) to deploy cloud-scanner as a serverless application for this purpose. See [Deploy cloud scanner as a serverless application](deploy-sls.md).

Refer to this example to get started with scrapping and displaying metrics:  [GitHub - demeringo/cloud-scanner-prom-config: A basic prometheus config to scrape metrics from Boavizta Cloud scanner](https://github.com/demeringo/cloud-scanner-prom-config). It provides a basic prometheus srape config and a sample dashboard.
