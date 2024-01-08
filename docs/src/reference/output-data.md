# Output data

Cloud scanner CLI and serverless application return data as _json_ or _Open Metrics_ (Prometheus) format.

## JSON CLI output (the default)

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

## Server mode json results

The format of the json results is slightly more complex in server mode.

When run in server mode, the server exposes an OpenAPI specification at <http://127.0.0.1:8000/openapi.json> and a swagger-ui:
<http://127.0.0.1:8000/swagger-ui/>

See  [OpenAPI specification in server mode](./openapi-server-mode.md)

## OpenMetrics/Prometheus output

As CLI application, If using `--as-metrics` or `-m`  option or the `serve` command, cloud-scanner returns consolidated results as OpenMetric/Prometheus format instead of json details.
This is also the default format of the serverless app `metrics` route.

doWhen using the metric output format, you get 2 sets of metrics

- Metrics named: _boavizta_xxxxx_ are _summary_ metrics (total number of resources, summed impacts, a.s.o)
- Metrics named _boavizta_resource_yyy_ are specific to individual resources. The metric label can be filtered to identify resource.

```sh
cargo run -- --as-metrics estimate -u 1 
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
