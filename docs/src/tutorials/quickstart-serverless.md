#  Run as serverless  ⚡

This will deploy the cloud-scanner _inside_ your AWS account. You can use it to 
- scan the account to get json impacts (as you would do with the CLI)
- or
- get metrics (that you can scrape with prometheus or the monitoring system of your choice)


The application is build and deployed using the serverless framework (see [serverless-design](../reference/serverless-design.md)).

## Prerequisites

The deployment creates an aws role configured to get sufficient permission to scan your resources without requesting end-user authentication.

1. Nodejs installed locally
2. An AWS account/profile with sufficient permissions to deploy lambda, configure API gateway, and create a role.

⚠ The deployment process was only tested on Linux.

## Deploy the application ⚡

```sh
# Install node the serverless framework and it's dependencies
npm i
export aws_profile = <my profile>
# Deploy the application to your AWS account
serverless deploy
```

## Usage

### Scan the account / region 💻

This returns results in json format (see below, same as CLI)

https://xxxxx.execute-api.eu-west-1.amazonaws.com/dev/scan?hours_use_time=5&aws_region=eu-west-1

Use `hours_use_time` and `aws_region` parameters in the query to pass the values to the lambda.

### Get Metrics 📊

https://xxxxx.execute-api.eu-west-1.amazonaws.com/dev/metrics?aws_region=eu-central-1

Returns metrics for **1 hour of use** in prometheus format.

The metrics represent the costs / impacts of one hour of use of the resources present in your account at the time of the query.

Use `aws_region` parameters in the query.

```
# HELP boavizta_number_of_instances_total Number of instances detected during the scan.
# TYPE boavizta_number_of_instances_total gauge
boavizta_number_of_instances_total{awsregion="eu-central-1",country="DEU"} 7
# HELP boavizta_number_of_instances_assessed Number of instances that were considered in the measure.
# TYPE boavizta_number_of_instances_assessed gauge
boavizta_number_of_instances_assessed{awsregion="eu-central-1",country="DEU"} 5
# HELP boavizta_duration_of_use_hours Number of instances detected during the scan.
# TYPE boavizta_duration_of_use_hours gauge
boavizta_duration_of_use_hours{awsregion="eu-central-1",country="DEU"} 1.0
# HELP boavizta_pe_manufacture_megajoules Power consumed for manufacture.
# TYPE boavizta_pe_manufacture_megajoules gauge
boavizta_pe_manufacture_megajoules{awsregion="eu-central-1",country="DEU"} 1760.0
# HELP boavizta_pe_use_megajoules Power consumed during usage.
# TYPE boavizta_pe_use_megajoules gauge
boavizta_pe_use_megajoules{awsregion="eu-central-1",country="DEU"} 0.86
# EOF
```
