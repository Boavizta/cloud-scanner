# Common issues

## The demo dashboard does not show any metric...

Several reasons may explain why metrics do not appear in the dashboard.

1. Verify in the docker-compose logs that there is no error related to AWS authentication.
2. Check that prometheus is configured to retrieve metrics for the region you use.

⚠ The demo docker-compose is preconfigured to generate metrics only for the following regions:
- eu-west-1
- eu-west-3
- eu-central-1
- us-east-1

Refer to  [Generating metrics for additional regions](../how-to/set-up-dashboard.md#generating-metrics-for-additional-regions) to retrieve metrics for additional regions.

## The docker compose demo may not display the latest version of the Grafana dashboard

/!\ The existing Grafana volume is preserved even after deploying a newer version of the docker-compose demo stack.

As a consequence the Grafana provisioning does not occur. Any update of the demo dashboard provided with the new stack is not reflected in the instance, even if Grafana container itself is recreated.

### Workaround

We have 2 possibilities, depending if we want to preserve the existing Grafana configuration (volume) or not.

- *Preserve* the existing volume if it was previously customized to contain additional Grafana configuration (outside cloud scanner) or if you add custom dashboards.
- Delete the volume if you use a vanilla instance of cloud-scanner demo.

#### Updating the dashboard while keeping existing volume

Wee need to **upload the new dashboard** definition manually **using  the Grafana web UI.**
  
- The dashboard definition can be found under `dashboard-config/provisioning/dashboards/`: [grafana-dashboard-cloud-impacts.template.json](https://github.com/Boavizta/cloud-scanner/blob/main/dashboard-config/provisioning/dashboards/grafana-dashboard-cloud-impacts.template.json)
- By default Grafana UI is exposed at `http://<host-ip>:3001`

#### Updating the dashboard by recreating Grafana configuration from scratch

/!\ Use this method only if Grafana was **not** customized after deployment of the demo or you are ok to loose these changes.

The easiest is to delete the Grafana volume and Grafana container. They will be recreated, and Grafana provisioned with latest dashboard, when we restart the docker compose stack.

```sh
# stop docker compose stack
docker compose down
# delete Grafana container (necessary because it may be using the volume even if stopped)
docker rm grafana_boa
# now delete volume
docker volume rm cloud-scanner_grafana-data
# Recreate compose stack
docker compose up
```
