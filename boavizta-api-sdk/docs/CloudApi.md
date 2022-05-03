# \CloudApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**instance_cloud_impact_v1_cloud_aws_post**](CloudApi.md#instance_cloud_impact_v1_cloud_aws_post) | **POST** /v1/cloud/aws | Instance Cloud Impact
[**server_get_all_archetype_name_v1_cloud_aws_all_instances_get**](CloudApi.md#server_get_all_archetype_name_v1_cloud_aws_all_instances_get) | **GET** /v1/cloud/aws/all_instances | Server Get All Archetype Name



## instance_cloud_impact_v1_cloud_aws_post

> serde_json::Value instance_cloud_impact_v1_cloud_aws_post(instance_type, verbose, usage_cloud)
Instance Cloud Impact

# ✔️AWS instance impacts from instance type and usage  ### 📋 Instance type  AWS name of the chosen instance. You can retrieve the [list here](#/cloud/server_get_all_archetype_name_v1_cloud_all_aws_instances_get). ### 👄 Verbose If set at true, shows the impacts of each components and the value used for each attributes    ### ⏲ Duration Usage impacts are given for a specific time duration. Duration can be given : | time unit | Usage parameter | |------|-----| | HOURS | ```hours_use_time``` | | DAYS | ```days_use_time``` | | YEARS | ```years_use_time``` | *Note* : units are cumulative ### 🧮 Measure  🔨 Manufacture impacts are the sum of the pre-registered components impacts divided by the number of instances host in the physicall server  🔌 Usage impacts are measured by multiplying : * a **duration**  * an **impact factor** (```gwp_factor```, ```pe_factor```, ```adp_factor```) - retrieve with ```usage_location``` if not given  * The ```time``` per load in ```workload``` object. The ```power``` per load is retreive from the ```instance_type```

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**instance_type** | Option<**String**> |  |  |
**verbose** | Option<**bool**> |  |  |[default to true]
**usage_cloud** | Option<[**UsageCloud**](UsageCloud.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## server_get_all_archetype_name_v1_cloud_aws_all_instances_get

> serde_json::Value server_get_all_archetype_name_v1_cloud_aws_all_instances_get()
Server Get All Archetype Name

# ✔️Get all the available aws instances Return the name of all pre-registered aws instances

### Parameters

This endpoint does not need any parameter.

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

