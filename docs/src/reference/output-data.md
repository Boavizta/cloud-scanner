# Output data

Cloud scanner CLI and serverless application return data as _json_ or _Open Metrics_ (Prometheus) format.

## JSON CLI impacts output

Cloud scanner returns a json array of instances metadata.

⚠ Returns _empty_ impacts when the _instance type_ is not known in Boavizta database

```json
{
  "impacting_resources": [
    {
      "cloud_resource": {
        "provider": "AWS",
        "id": "instance-F",
        "location": {
          "aws_region": "eu-west-1",
          "iso_country_code": "IRL"
        },
        "resource_details": {
          "instance": {
            "instance_type": "FICTIVE-INSTANCE-TYPE",
            "usage": {
              "average_cpu_load": 100,
              "state": "running"
            }
          }
        },
        "tags": []
      },
      "impacts_values": null,
      "impacts_duration_hours": 5
    },
    {
      "cloud_resource": {
        "provider": "AWS",
        "id": "instance-1",
        "location": {
          "aws_region": "eu-west-1",
          "iso_country_code": "IRL"
        },
        "resource_details": {
          "instance": {
            "instance_type": "a1.medium",
            "usage": {
              "average_cpu_load": 0.3,
              "state": "running"
            }
          }
        },
        "tags": []
      },
      "impacts_values": {
        "adp_manufacture_kgsbeq": 8.8e-07,
        "adp_use_kgsbeq": 1.29e-10,
        "pe_manufacture_megajoules": 0.057,
        "pe_use_megajoules": 0.063,
        "gwp_manufacture_kgco2eq": 0.0041,
        "gwp_use_kgco2eq": 0.00226,
        "raw_data": {
          "impacts": {
            "adp": {
              "description": "Use of minerals and fossil ressources",
              "embedded": {
                "max": 1.261e-06,
                "min": 5.808e-07,
                "value": 8.8e-07,
                "warnings": [
                  "End of life is not included in the calculation"
                ]
              },
              "unit": "kgSbeq",
              "use": {
                "max": 1.552e-10,
                "min": 1.164e-10,
                "value": 1.29e-10
              }
            },
            "gwp": {
              "description": "Total climate change",
              "embedded": {
                "max": 0.005669,
                "min": 0.002324,
                "value": 0.0041,
                "warnings": [
                  "End of life is not included in the calculation"
                ]
              },
              "unit": "kgCO2eq",
              "use": {
                "max": 0.002718,
                "min": 0.002038,
                "value": 0.00226
              }
            },
            "pe": {
              "description": "Consumption of primary energy",
              "embedded": {
                "max": 0.07877,
                "min": 0.03178,
                "value": 0.057,
                "warnings": [
                  "End of life is not included in the calculation"
                ]
              },
              "unit": "MJ",
              "use": {
                "max": 0.07577,
                "min": 0.05683,
                "value": 0.063
              }
            }
          }
        }
      },
      "impacts_duration_hours": 5
    }
  ]
}
```

## Schema of the json output

A schema described the format of the JSON output  (as part of theOpenAPI specification).

To access it:

1. start a server (`cloud-scanner-cli serve`)
2. access the OpenAPI specification at <http://127.0.0.1:8000/openapi.json> and a swagger-ui at
<http://127.0.0.1:8000/swagger-ui/>

See  [OpenAPI specification in server mode](./openapi-server-mode.md)

## OpenMetrics/Prometheus output

As CLI application, when using the  `metrics` or  `serve` command, cloud-scanner returns consolidated results as OpenMetric/Prometheus format instead of json.
This is also the default format of the serverless application `metrics` route.

When using the metric output format, you get 2 sets of metrics

- Metrics named _boavizta_xxxxx_ are _summary_ metrics (total number of resources, summed impacts, a.s.o)
- Metrics named _boavizta_resource_yyy_ are specific to _individual resources_. The metric labels can be filtered to identify resource.

```sh
cargo run metrics -u 1
```

Returns:

```sh
# HELP boavizta_number_of_resources_total Number of resources detected during the inventory.
# TYPE boavizta_number_of_resources_total gauge
boavizta_number_of_resources_total{awsregion="eu-west-1",country="IRL"} 4
# HELP boavizta_number_of_resources_assessed Number of resources that were considered in the estimation of impacts.
# TYPE boavizta_number_of_resources_assessed gauge
boavizta_number_of_resources_assessed{awsregion="eu-west-1",country="IRL"} 4
# HELP boavizta_duration_of_use_hours Use duration considered to estimate impacts.
# TYPE boavizta_duration_of_use_hours gauge
boavizta_duration_of_use_hours{awsregion="eu-west-1",country="IRL"} 1.0
# HELP boavizta_pe_manufacture_megajoules Energy consumed for manufacture.
# TYPE boavizta_pe_manufacture_megajoules gauge
boavizta_pe_manufacture_megajoules{awsregion="eu-west-1",country="IRL"} 0.0704
# HELP boavizta_pe_use_megajoules Energy consumed during use.
# TYPE boavizta_pe_use_megajoules gauge
boavizta_pe_use_megajoules{awsregion="eu-west-1",country="IRL"} 0.2636
# HELP boavizta_adp_manufacture_kgsbeq Abiotic resources depletion potential of manufacture.
# TYPE boavizta_adp_manufacture_kgsbeq gauge
boavizta_adp_manufacture_kgsbeq{awsregion="eu-west-1",country="IRL"} 8.3e-7
# HELP boavizta_adp_use_kgsbeq Abiotic resources depletion potential of use.
# TYPE boavizta_adp_use_kgsbeq gauge
boavizta_adp_use_kgsbeq{awsregion="eu-west-1",country="IRL"} 5.387e-10
# HELP boavizta_gwp_manufacture_kgco2eq Global Warming Potential of manufacture.
# TYPE boavizta_gwp_manufacture_kgco2eq gauge
boavizta_gwp_manufacture_kgco2eq{awsregion="eu-west-1",country="IRL"} 0.00532
# HELP boavizta_gwp_use_kgco2eq Global Warming Potential of use.
# TYPE boavizta_gwp_use_kgco2eq gauge
boavizta_gwp_use_kgco2eq{awsregion="eu-west-1",country="IRL"} 0.009430000000000001
# HELP boavizta_resource_duration_of_use_hours Use duration considered to estimate impacts.
# TYPE boavizta_resource_duration_of_use_hours gauge
boavizta_resource_duration_of_use_hours{awsregion="eu-west-1",country="IRL",resource_type="Instance",resource_id="i-033df52f12f30ca66",resource_tags="[CloudResourceTag { key: "Name", value: Some("test-boavizta") }, CloudResourceTag { key: "CustomTagNameForDebug", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "CreatorName", value: Some("olivierdemeringoadm") }]",resource_state="Stopped"} 1.0
boavizta_resource_duration_of_use_hours{awsregion="eu-west-1",country="IRL",resource_type="Instance",resource_id="i-0a3e6b8cdb50c49b8",resource_tags="[CloudResourceTag { key: "CustomTagNameForDebug", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "appname", value: Some("app1") }, CloudResourceTag { key: "created_by", value: Some("demeringo") }, CloudResourceTag { key: "CreatorName", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "Name", value: Some("boavizta-c5n.xlarge") }]",resource_state="Stopped"} 1.0
boavizta_resource_duration_of_use_hours{awsregion="eu-west-1",country="IRL",resource_type="Instance",resource_id="i-03c8f84a6318a8186",resource_tags="[CloudResourceTag { key: "Name", value: Some("test-boapi") }, CloudResourceTag { key: "CreatorName", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "CustomTagNameForDebug", value: Some("olivierdemeringoadm") }]",resource_state="Running"} 1.0
boavizta_resource_duration_of_use_hours{awsregion="eu-west-1",country="IRL",resource_type="Instance",resource_id="i-003ea8da7bb9bfff9",resource_tags="[CloudResourceTag { key: "CustomTagNameForDebug", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "CreatorName", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "Name", value: Some("test-boavizta-2") }]",resource_state="Running"} 1.0
# HELP boavizta_resource_pe_embedded_megajoules Energy consumed for manufacture.
# TYPE boavizta_resource_pe_embedded_megajoules gauge
boavizta_resource_pe_embedded_megajoules{awsregion="eu-west-1",country="IRL",resource_type="Instance",resource_id="i-003ea8da7bb9bfff9",resource_tags="[CloudResourceTag { key: "CustomTagNameForDebug", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "CreatorName", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "Name", value: Some("test-boavizta-2") }]",resource_state="Running"} 0.021
boavizta_resource_pe_embedded_megajoules{awsregion="eu-west-1",country="IRL",resource_type="Instance",resource_id="i-0a3e6b8cdb50c49b8",resource_tags="[CloudResourceTag { key: "CustomTagNameForDebug", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "appname", value: Some("app1") }, CloudResourceTag { key: "created_by", value: Some("demeringo") }, CloudResourceTag { key: "CreatorName", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "Name", value: Some("boavizta-c5n.xlarge") }]",resource_state="Stopped"} 0.017
boavizta_resource_pe_embedded_megajoules{awsregion="eu-west-1",country="IRL",resource_type="Instance",resource_id="i-03c8f84a6318a8186",resource_tags="[CloudResourceTag { key: "Name", value: Some("test-boapi") }, CloudResourceTag { key: "CreatorName", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "CustomTagNameForDebug", value: Some("olivierdemeringoadm") }]",resource_state="Running"} 0.0114
boavizta_resource_pe_embedded_megajoules{awsregion="eu-west-1",country="IRL",resource_type="Instance",resource_id="i-033df52f12f30ca66",resource_tags="[CloudResourceTag { key: "Name", value: Some("test-boavizta") }, CloudResourceTag { key: "CustomTagNameForDebug", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "CreatorName", value: Some("olivierdemeringoadm") }]",resource_state="Stopped"} 0.021
# HELP boavizta_resource_pe_use_megajoules Energy consumed during use.
# TYPE boavizta_resource_pe_use_megajoules gauge
boavizta_resource_pe_use_megajoules{awsregion="eu-west-1",country="IRL",resource_type="Instance",resource_id="i-003ea8da7bb9bfff9",resource_tags="[CloudResourceTag { key: "CustomTagNameForDebug", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "CreatorName", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "Name", value: Some("test-boavizta-2") }]",resource_state="Running"} 0.082
boavizta_resource_pe_use_megajoules{awsregion="eu-west-1",country="IRL",resource_type="Instance",resource_id="i-03c8f84a6318a8186",resource_tags="[CloudResourceTag { key: "Name", value: Some("test-boapi") }, CloudResourceTag { key: "CreatorName", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "CustomTagNameForDebug", value: Some("olivierdemeringoadm") }]",resource_state="Running"} 0.0126
boavizta_resource_pe_use_megajoules{awsregion="eu-west-1",country="IRL",resource_type="Instance",resource_id="i-033df52f12f30ca66",resource_tags="[CloudResourceTag { key: "Name", value: Some("test-boavizta") }, CloudResourceTag { key: "CustomTagNameForDebug", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "CreatorName", value: Some("olivierdemeringoadm") }]",resource_state="Stopped"} 0.081
boavizta_resource_pe_use_megajoules{awsregion="eu-west-1",country="IRL",resource_type="Instance",resource_id="i-0a3e6b8cdb50c49b8",resource_tags="[CloudResourceTag { key: "CustomTagNameForDebug", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "appname", value: Some("app1") }, CloudResourceTag { key: "created_by", value: Some("demeringo") }, CloudResourceTag { key: "CreatorName", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "Name", value: Some("boavizta-c5n.xlarge") }]",resource_state="Stopped"} 0.088
# HELP boavizta_resource_adp_embedded_kgsbeq Abiotic resources depletion potential of embedded impacts.
# TYPE boavizta_resource_adp_embedded_kgsbeq gauge
boavizta_resource_adp_embedded_kgsbeq{awsregion="eu-west-1",country="IRL",resource_type="Instance",resource_id="i-033df52f12f30ca66",resource_tags="[CloudResourceTag { key: "Name", value: Some("test-boavizta") }, CloudResourceTag { key: "CustomTagNameForDebug", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "CreatorName", value: Some("olivierdemeringoadm") }]",resource_state="Stopped"} 2.1e-7
boavizta_resource_adp_embedded_kgsbeq{awsregion="eu-west-1",country="IRL",resource_type="Instance",resource_id="i-003ea8da7bb9bfff9",resource_tags="[CloudResourceTag { key: "CustomTagNameForDebug", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "CreatorName", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "Name", value: Some("test-boavizta-2") }]",resource_state="Running"} 2.1e-7
boavizta_resource_adp_embedded_kgsbeq{awsregion="eu-west-1",country="IRL",resource_type="Instance",resource_id="i-03c8f84a6318a8186",resource_tags="[CloudResourceTag { key: "Name", value: Some("test-boapi") }, CloudResourceTag { key: "CreatorName", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "CustomTagNameForDebug", value: Some("olivierdemeringoadm") }]",resource_state="Running"} 1.8e-7
boavizta_resource_adp_embedded_kgsbeq{awsregion="eu-west-1",country="IRL",resource_type="Instance",resource_id="i-0a3e6b8cdb50c49b8",resource_tags="[CloudResourceTag { key: "CustomTagNameForDebug", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "appname", value: Some("app1") }, CloudResourceTag { key: "created_by", value: Some("demeringo") }, CloudResourceTag { key: "CreatorName", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "Name", value: Some("boavizta-c5n.xlarge") }]",resource_state="Stopped"} 2.3e-7
# HELP boavizta_resource_adp_use_kgsbeq Abiotic resources depletion potential of use.
# TYPE boavizta_resource_adp_use_kgsbeq gauge
boavizta_resource_adp_use_kgsbeq{awsregion="eu-west-1",country="IRL",resource_type="Instance",resource_id="i-0a3e6b8cdb50c49b8",resource_tags="[CloudResourceTag { key: "CustomTagNameForDebug", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "appname", value: Some("app1") }, CloudResourceTag { key: "created_by", value: Some("demeringo") }, CloudResourceTag { key: "CreatorName", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "Name", value: Some("boavizta-c5n.xlarge") }]",resource_state="Stopped"} 1.81e-10
boavizta_resource_adp_use_kgsbeq{awsregion="eu-west-1",country="IRL",resource_type="Instance",resource_id="i-033df52f12f30ca66",resource_tags="[CloudResourceTag { key: "Name", value: Some("test-boavizta") }, CloudResourceTag { key: "CustomTagNameForDebug", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "CreatorName", value: Some("olivierdemeringoadm") }]",resource_state="Stopped"} 1.65e-10
boavizta_resource_adp_use_kgsbeq{awsregion="eu-west-1",country="IRL",resource_type="Instance",resource_id="i-03c8f84a6318a8186",resource_tags="[CloudResourceTag { key: "Name", value: Some("test-boapi") }, CloudResourceTag { key: "CreatorName", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "CustomTagNameForDebug", value: Some("olivierdemeringoadm") }]",resource_state="Running"} 2.57e-11
boavizta_resource_adp_use_kgsbeq{awsregion="eu-west-1",country="IRL",resource_type="Instance",resource_id="i-003ea8da7bb9bfff9",resource_tags="[CloudResourceTag { key: "CustomTagNameForDebug", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "CreatorName", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "Name", value: Some("test-boavizta-2") }]",resource_state="Running"} 1.67e-10
# HELP boavizta_resource_gwp_embedded_kgco2eq Global Warming Potential of embedded impacts.
# TYPE boavizta_resource_gwp_embedded_kgco2eq gauge
boavizta_resource_gwp_embedded_kgco2eq{awsregion="eu-west-1",country="IRL",resource_type="Instance",resource_id="i-003ea8da7bb9bfff9",resource_tags="[CloudResourceTag { key: "CustomTagNameForDebug", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "CreatorName", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "Name", value: Some("test-boavizta-2") }]",resource_state="Running"} 0.0016
boavizta_resource_gwp_embedded_kgco2eq{awsregion="eu-west-1",country="IRL",resource_type="Instance",resource_id="i-033df52f12f30ca66",resource_tags="[CloudResourceTag { key: "Name", value: Some("test-boavizta") }, CloudResourceTag { key: "CustomTagNameForDebug", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "CreatorName", value: Some("olivierdemeringoadm") }]",resource_state="Stopped"} 0.0016
boavizta_resource_gwp_embedded_kgco2eq{awsregion="eu-west-1",country="IRL",resource_type="Instance",resource_id="i-0a3e6b8cdb50c49b8",resource_tags="[CloudResourceTag { key: "CustomTagNameForDebug", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "appname", value: Some("app1") }, CloudResourceTag { key: "created_by", value: Some("demeringo") }, CloudResourceTag { key: "CreatorName", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "Name", value: Some("boavizta-c5n.xlarge") }]",resource_state="Stopped"} 0.0013
boavizta_resource_gwp_embedded_kgco2eq{awsregion="eu-west-1",country="IRL",resource_type="Instance",resource_id="i-03c8f84a6318a8186",resource_tags="[CloudResourceTag { key: "Name", value: Some("test-boapi") }, CloudResourceTag { key: "CreatorName", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "CustomTagNameForDebug", value: Some("olivierdemeringoadm") }]",resource_state="Running"} 0.00082
# HELP boavizta_resource_gwp_use_kgco2eq Global Warming Potential of use.
# TYPE boavizta_resource_gwp_use_kgco2eq gauge
boavizta_resource_gwp_use_kgco2eq{awsregion="eu-west-1",country="IRL",resource_type="Instance",resource_id="i-033df52f12f30ca66",resource_tags="[CloudResourceTag { key: "Name", value: Some("test-boavizta") }, CloudResourceTag { key: "CustomTagNameForDebug", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "CreatorName", value: Some("olivierdemeringoadm") }]",resource_state="Stopped"} 0.00289
boavizta_resource_gwp_use_kgco2eq{awsregion="eu-west-1",country="IRL",resource_type="Instance",resource_id="i-0a3e6b8cdb50c49b8",resource_tags="[CloudResourceTag { key: "CustomTagNameForDebug", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "appname", value: Some("app1") }, CloudResourceTag { key: "created_by", value: Some("demeringo") }, CloudResourceTag { key: "CreatorName", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "Name", value: Some("boavizta-c5n.xlarge") }]",resource_state="Stopped"} 0.00317
boavizta_resource_gwp_use_kgco2eq{awsregion="eu-west-1",country="IRL",resource_type="Instance",resource_id="i-03c8f84a6318a8186",resource_tags="[CloudResourceTag { key: "Name", value: Some("test-boapi") }, CloudResourceTag { key: "CreatorName", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "CustomTagNameForDebug", value: Some("olivierdemeringoadm") }]",resource_state="Running"} 0.00045
boavizta_resource_gwp_use_kgco2eq{awsregion="eu-west-1",country="IRL",resource_type="Instance",resource_id="i-003ea8da7bb9bfff9",resource_tags="[CloudResourceTag { key: "CustomTagNameForDebug", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "CreatorName", value: Some("olivierdemeringoadm") }, CloudResourceTag { key: "Name", value: Some("test-boavizta-2") }]",resource_state="Running"} 0.00292
# EOF
```
