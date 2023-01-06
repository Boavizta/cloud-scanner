# Quick start : display dashboard using docker-compose 🐳

No installation needed, you will run a public docker image of cloud-scanner CLI and Boavizta API.

All data remain local (this docker-compose stack uses a _private instance_ of Boavizta API).

## Pre-requisites

- Docker and docker-compose
- A working AWS account (and your AWS CLI profile already configured)

![components of monitoring stack in docker compose](../images/cloud-scanner-metrics-compose.excalidraw.png)


## Run the demo dashboard

```sh
# Map your AWS credentials
export AWS_PROFILE=name-of-your-profile
# Start the stack (from the root of the repository)
docker-compose up
```

- Demo dashboard is exposed on http://localhost:3001 
- Log in with user admin/admin
- Select the dashboard in the left menu.

![Demo dashboard exposing cloud scanner metrics](../images/CS-dashboard.png "A example dashboard rendering cloud scanner metrics")

## Additional info

- ⚠ This docker-compose example is **not** intended  for production deployment, but rather for quick testing.
  - ports of all services are exposed.
  - Grafana is served on http with default login.
- You may have to update the line mapping your AWS profile (Replace `AWS_PROFILE=${AWS_PROFILE}` by `AWS_PROFILE=the-real-name-of-your-profile`).
- In corporate environments, you may need to provide your certificates authorities certificates (`ca-certificates`) to the cloud-scanner container (uncomment the mapping line in the docker-compose file).
- For te demo, we deliberately set a short metrics scrapping interval (30 seconds in this demo). In production deloymnent, you may want to increase this metric scraping interval in the prometheus configuration file.
