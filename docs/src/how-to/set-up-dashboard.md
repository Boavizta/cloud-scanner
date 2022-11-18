# Setup monitoring dashboard

Cloud-scanner can be used to export metrics related to the impact of your cloud account.

The metrics are exposed in Prometheus/OpenMetrics format. You can use it to feed a monitoring dashboard and keep an eye on the evolution of impacts of your usage.

![cloud-scanner-metrics-dashboard](../images/cloud-scanner-dashboard-clear.png)

## Overview

1. Deploy cloud-scanner as a serverless app in your AWS account or start a cloud-scanner with the `--serve`  option
2. Setup Prometheus to scrape the metrics
3. Configure a grafana dashboard to display the results.

## Detailed steps

For production use It is easier (and safer) to deploy cloud-scanner as a serverless application. See [Deploy cloud scanner as a serverless application](deploy-sls.md).

An example Grafana dashboard is provided as part of the docker-compose demo.

Prometheus and Grafana config files are in the [dashboard-config](https://github.com/Boavizta/cloud-scanner/tree/main/dashboard-config) directory.

Additional info:

- ⚠ The docker-compose is **not** intended  for production deployment but rather for quick testing.
  - ports of all services are exposed.
  - grafana is served on http.
- You may have to update the line mapping your AWS profile (Replace `AWS_PROFILE=${AWS_PROFILE}` by `AWS_PROFILE=the-real-name-of-your-profile`).
- In corporate environments, you may need to provide your certificates authorities certificates (`ca-certificates`) to the cloud-scanner container (uncomment the mapping line in the docker-compose file).
