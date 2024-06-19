# How we estimate instance workload

Workload (or intensity of use) of instances is estimated using CPU load level as a proxy.

## Estimating instances workload for AWS

The CPU load of AWS instances is retrieved using _AWS Cloudwatch CPU metrics summary_.

Cloud scanner uses a sampling period of 15 minutes, but impacts metrics are returned as impacts equivalent to one hour of use.

This means that instance impacts metrics data returned can be understood as: `impact for one hour of use (considering  the CPU workload of 15 last minutes)`.

Why this default sampling period of 15 minutes ?

- It is sufficient for our current monitoring needs (but maybe we may make this setting configurable in the future).
- It is hard to go below 10 minutes. Default (and free) period of AWS instance metrics is 5 minutes. You need to activate `detailed monitoring` (extra feature) for 1 minute granularity: [List the available CloudWatch metrics for your instances - Amazon Elastic Compute Cloud](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/viewing_metrics_with_cloudwatch.html#ec2-cloudwatch-metrics)).

## Estimating instances workload for Azure

Not implemented.
