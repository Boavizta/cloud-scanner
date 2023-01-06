# Integrate metrics to a monitoring dashboard

Cloud-scanner can be used to export metrics related to the impact of your cloud account.

The metrics are exposed in Prometheus/OpenMetrics format. You can use it to feed a monitoring dashboard and keep an eye on the evolution of impacts of your usage.

![cloud-scanner-metrics-dashboard](../images/cloud-scanner-dashboard-clear.png)

## Overview

1. Start cloud-scanner in metrics mode (using the  `--serve`  option or by deploying it as a serverless application).
2. Setup Prometheus to scrape the metrics
3. Configure a dashboard to display the results.

![components of monitoring stack in docker compose](../images/cloud-scanner-metrics-compose.excalidraw.png)

## Detailed steps

You can refer to  the provided [docker compose example ](../tutorials/quickstart-dashboard-docker.md) for a quick start.

Prometheus and Grafana config files are in the [dashboard-config](https://github.com/Boavizta/cloud-scanner/tree/main/dashboard-config) directory.

For production use:

- It is easier (and safer) to deploy cloud-scanner as a serverless application. See [Deploy cloud scanner as a serverless application](deploy-sls.md).
- In production environment, you may want to increase the  metrics scraping interval (30 seconds in this demo) in the prometheus configuration file.
