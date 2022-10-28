# How we process workload

Workload of instances are estimated using AWS cloudwatch CPU metrics summary.

Cloud scanner uses a sampling period of 15 minutes, but impacts metrics are returned as impacts equivalent to one hour of use.

This means that instance impacts metrics data returned can be understood as: `impact for one hour of use (considering  the CPU workload of 15 last minutes)`.

Why this default sampling period of 15 minutes ?

- It seems sufficient for our current monitoring needs (but maybe we can make it configurable in the future).
- It seems hard to go below 10 minutes (because default period  of AWS instance metrics is 5 minutes. You need to activate `detailed monitoring` (extra feature) for 1 minute granularity: [List the available CloudWatch metrics for your instances - Amazon Elastic Compute Cloud](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/viewing_metrics_with_cloudwatch.html#ec2-cloudwatch-metrics)).
