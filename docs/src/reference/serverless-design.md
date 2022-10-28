# Serverless design

We use the [serverless framework](https://www.serverless.com/) and the [softprops/serverless-rust](https://github.com/softprops/serverless-rust) plugin to ease packaging and deployment as lambda.

The cloud-scanner-cli is wrapped into a set of lambdas functions exposed behind an AWS API gateway.

_This is certainly not the only way to deploy the application. If you want more control, you could compile, package and deploy the application with Terraform or CDK, but this is not documented yet._


## Serverless routes

### Scan account / region

Returns results in json format (see below, same as CLI)

https://xxxxx.execute-api.eu-west-1.amazonaws.com/dev/scan?hours_use_time=5&aws_region=eu-west-1

Use `hours_use_time` and `aws_region` parameters in the query

### Get Metrics

https://xxxxx.execute-api.eu-west-1.amazonaws.com/dev/metrics?aws_region=eu-central-1

returns metrics for one hour of use in prometheus format.
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