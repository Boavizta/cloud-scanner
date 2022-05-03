# \ServerApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**server_get_all_archetype_name_v1_server_all_default_models_get**](ServerApi.md#server_get_all_archetype_name_v1_server_all_default_models_get) | **GET** /v1/server/all_default_models | Server Get All Archetype Name
[**server_impact_by_config_v1_server_post**](ServerApi.md#server_impact_by_config_v1_server_post) | **POST** /v1/server/ | Server Impact By Config
[**server_impact_by_model_v1_server_model_get**](ServerApi.md#server_impact_by_model_v1_server_model_get) | **GET** /v1/server/model | Server Impact By Model



## server_get_all_archetype_name_v1_server_all_default_models_get

> serde_json::Value server_get_all_archetype_name_v1_server_all_default_models_get()
Server Get All Archetype Name

# ✔️Get all the available server models Return the name of all pre-registered server models

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


## server_impact_by_config_v1_server_post

> serde_json::Value server_impact_by_config_v1_server_post(verbose, server_dto)
Server Impact By Config

# ✔️Server impacts from configuration Retrieve the impacts of a given server configuration. ### 💡 Smart complete All missing components and components attributes are retrieve with the closest available values. If no data are available default maximizing data are used  ### 👄 Verbose If set at true, shows the impacts of each components and the value used for each attributes   ### 📋 Archetype An archetype is a pre-registered server model. An ```archetype``` can be specify in the model object. In case an archetype is specified, all missing data are retrieve from the archetype. You can have a list of available archetype's server models [here](#/server/server_get_all_archetype_name_v1_server_all_default_models_get)   ### ⏲ Duration Usage impacts are given for a specific time duration. Duration can be given in : | time unit | Usage parameter | |------|-----| | HOURS | ```hours_use_time``` | | DAYS | ```days_use_time``` | | YEARS | ```years_use_time``` | If no duration is given, **the impact is measured for a year**. *Note* : units are cumulative ### 🧮 Measure 🔨 Manufacture impacts are the sum of the components impacts  🔌 Usage impacts are measured by multiplying : * a **duration**  * an **impact factor** (```gwp_factor```, ```pe_factor```, ```adp_factor```) - retrieve with ```usage_location``` if not given  * an **electrical consumption** (```hours_electrical_consumption```) - retrieve with ```workload``` if not given

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verbose** | Option<**bool**> |  |  |[default to true]
**server_dto** | Option<[**ServerDto**](ServerDto.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## server_impact_by_model_v1_server_model_get

> serde_json::Value server_impact_by_model_v1_server_model_get(archetype, verbose)
Server Impact By Model

# ✔️Server impacts from model name Retrieve the impacts of a given server name (archetype). ### 📋 Model Uses the [classic server impacts router](#/server/server_impact_by_config_v1_server__post) with a pre-registered model  ### 👄 Verbose If set at true, shows the impacts of each components and the value used for each attributes   ### 📋 Model name You can have a list of available server models names [here](#/server/server_get_all_archetype_name_v1_server_all_default_models_get)   ### 🧮 Measure 🔨 Manufacture impacts are the sum of the pre-registered components impacts  🔌 Usage impacts are measured based on the electrical consumption of the pre-registered model for a year 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**archetype** | Option<**String**> |  |  |
**verbose** | Option<**bool**> |  |  |[default to true]

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

