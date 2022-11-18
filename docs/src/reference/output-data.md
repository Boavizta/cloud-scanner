# Output data

Cloud scanner CLI and serverless application return data as _json_ or _Open Metrics_ (Prometheus) format.

## JSON output (the default)

Cloud scanner returns a json array of instances metadata.

⚠ Returns _empty_ impacts when the _instance type_ is not known in Boavizta database

```json
[
  {
    "instance_id": "i-001dc0ebbf9cb25c0",
    "instance_type": "t2.micro",
    "usage_data": {
      "hours_use_time": 5,
      "usage_location": "IRL"
    },
    "impacts": {}
  },
  {
    "instance_id": "i-004599844f7c24814",
    "instance_type": "t2.small",
    "usage_data": {
      "hours_use_time": 5,
      "usage_location": "IRL"
    },
    "impacts": {}
  },
  {
    "instance_id": "i-075444d7293d8bd76",
    "instance_type": "t2.micro",
    "usage_data": {
      "hours_use_time": 5,
      "usage_location": "IRL"
    },
    "impacts": {}
  },
  {
    "instance_id": "i-033df52f12f30ca66",
    "instance_type": "m6g.xlarge",
    "usage_data": {
      "hours_use_time": 5,
      "usage_location": "IRL"
    },
    "impacts": {
      "adp": {
        "manufacture": 0.0084,
        "unit": "kgSbeq",
        "use": 1.7e-9
      },
      "gwp": {
        "manufacture": 87,
        "unit": "kgCO2eq",
        "use": 0.029
      },
      "pe": {
        "manufacture": 1100,
        "unit": "MJ",
        "use": 0.82
      }
    }
  }
]
```

## OpenMetrics/Prometheus output

As CLI application, If using `--as-metrics` or `-m`  option or the `serve` command, cloud-scanner returns consolidated results as OpenMetric/Prometheus format instead of json details.
This is also the default format of the serverless app `metrics` route.

When using the metric output format, you cannot see the individual impacts of each instance. Instead, impacts of all instances are added to provide a global figure.

```sh
cargo run -- --as-metrics  standard -u 1

# HELP boavizta_number_of_instances_total Number of instances detected during the scan.
# TYPE boavizta_number_of_instances_total gauge
boavizta_number_of_instances_total{awsregion="eu-west-1",country="IRL"} 9
# HELP boavizta_number_of_instances_assessed Number of instances that were considered in the measure.
# TYPE boavizta_number_of_instances_assessed gauge
boavizta_number_of_instances_assessed{awsregion="eu-west-1",country="IRL"} 6
# HELP boavizta_duration_of_use_hours Number of instances detected during the scan.
# TYPE boavizta_duration_of_use_hours gauge
boavizta_duration_of_use_hours{awsregion="eu-west-1",country="IRL"} 1.0
# HELP boavizta_pe_manufacture_megajoules Power consumed for manufacture.
# TYPE boavizta_pe_manufacture_megajoules gauge
boavizta_pe_manufacture_megajoules{awsregion="eu-west-1",country="IRL"} 2060.0
# HELP boavizta_pe_use_megajoules Power consumed during usage.
# TYPE boavizta_pe_use_megajoules gauge
boavizta_pe_use_megajoules{awsregion="eu-west-1",country="IRL"} 0.228
# EOF
```
