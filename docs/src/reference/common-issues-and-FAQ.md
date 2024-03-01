# Common issues and FAQ

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
