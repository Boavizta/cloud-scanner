# \ComponentApi

All URIs are relative to *http://localhost*

Method | HTTP request | Description
------------- | ------------- | -------------
[**case_impact_bottom_up_v1_component_case_post**](ComponentApi.md#case_impact_bottom_up_v1_component_case_post) | **POST** /v1/component/case | Case Impact Bottom Up
[**cpu_impact_bottom_up_v1_component_cpu_post**](ComponentApi.md#cpu_impact_bottom_up_v1_component_cpu_post) | **POST** /v1/component/cpu | Cpu Impact Bottom Up
[**disk_impact_bottom_up_v1_component_hdd_post**](ComponentApi.md#disk_impact_bottom_up_v1_component_hdd_post) | **POST** /v1/component/hdd | Disk Impact Bottom Up
[**disk_impact_bottom_up_v1_component_ssd_post**](ComponentApi.md#disk_impact_bottom_up_v1_component_ssd_post) | **POST** /v1/component/ssd | Disk Impact Bottom Up
[**motherboard_impact_bottom_up_v1_component_motherboard_post**](ComponentApi.md#motherboard_impact_bottom_up_v1_component_motherboard_post) | **POST** /v1/component/motherboard | Motherboard Impact Bottom Up
[**power_supply_impact_bottom_up_v1_component_power_supply_post**](ComponentApi.md#power_supply_impact_bottom_up_v1_component_power_supply_post) | **POST** /v1/component/power_supply | Power Supply Impact Bottom Up
[**ram_impact_bottom_up_v1_component_ram_post**](ComponentApi.md#ram_impact_bottom_up_v1_component_ram_post) | **POST** /v1/component/ram | Ram Impact Bottom Up



## case_impact_bottom_up_v1_component_case_post

> serde_json::Value case_impact_bottom_up_v1_component_case_post(verbose, case)
Case Impact Bottom Up

# ✔️Case impacts from configuration ### 💡 Smart complete All missing data are retrieve with the closest available values. If no data are available default maximizing data are used  ### 👄 Verbose If set at true, shows the the values used for each attribute*Components have no units since they represent a single instance of a component.* ### 🧮 Measure The impacts values are set by default

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verbose** | Option<**bool**> |  |  |[default to true]
**case** | Option<[**Case**](Case.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## cpu_impact_bottom_up_v1_component_cpu_post

> serde_json::Value cpu_impact_bottom_up_v1_component_cpu_post(verbose, cpu)
Cpu Impact Bottom Up

# ✔️CPU impacts from configuration ### 💡 Smart complete All missing data are retrieve with the closest available values. If no data are available default maximizing data are used  ### 👄 Verbose If set at true, shows the the values used for each attribute*Components have no units since they represent a single instance of a component.* ### 🧮 Measure <h3>cpu<sub>manuf<sub><em>criteria</em></sub></sub> = ( cpu<sub>core<sub>units</sub></sub> x cpu<sub>diesize</sub> + 0,491 ) x cpu<sub>manuf_die<sub><em>criteria</em></sub></sub> + cpu<sub>manuf_base<sub><em>criteria</em></sub></sub></h3> 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verbose** | Option<**bool**> |  |  |[default to true]
**cpu** | Option<[**Cpu**](Cpu.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disk_impact_bottom_up_v1_component_hdd_post

> serde_json::Value disk_impact_bottom_up_v1_component_hdd_post(verbose, disk)
Disk Impact Bottom Up

# ✔️HDD impacts from configuration ### 💡 Smart complete All missing data are retrieve with the closest available values. If no data are available default maximizing data are used  ### 👄 Verbose If set at true, shows the the values used for each attribute*Components have no units since they represent a single instance of a component.* ### 🧮 Measure The impacts values are set by default

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verbose** | Option<**bool**> |  |  |[default to true]
**disk** | Option<[**Disk**](Disk.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## disk_impact_bottom_up_v1_component_ssd_post

> serde_json::Value disk_impact_bottom_up_v1_component_ssd_post(verbose, disk)
Disk Impact Bottom Up

# ✔️SSD impacts from configuration ### 💡 Smart complete All missing data are retrieve with the closest available values. If no data are available default maximizing data are used  ### 👄 Verbose If set at true, shows the the values used for each attribute*Components have no units since they represent a single instance of a component.* ### 🧮 Measure <h3>ssd<sub>manuf<sub><em>criteria</em></sub></sub> =  ( ssd<sub>size</sub> ssd<sub>density</sub> ) x ssd<sub>manuf_die<sub><em>criteria</em></sub></sub> + ssd<sub>manuf_base<sub><em>criteria</em></sub></sub></h3> 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verbose** | Option<**bool**> |  |  |[default to true]
**disk** | Option<[**Disk**](Disk.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## motherboard_impact_bottom_up_v1_component_motherboard_post

> serde_json::Value motherboard_impact_bottom_up_v1_component_motherboard_post(verbose, mother_board)
Motherboard Impact Bottom Up

# ✔️Motherboard impacts from configuration ### 💡 Smart complete All missing data are retrieve with the closest available values. If no data are available default maximizing data are used  ### 👄 Verbose If set at true, shows the the values used for each attribute*Components have no units since they represent a single instance of a component.* ### 🧮 Measure The impacts values are set by default

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verbose** | Option<**bool**> |  |  |[default to true]
**mother_board** | Option<[**MotherBoard**](MotherBoard.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## power_supply_impact_bottom_up_v1_component_power_supply_post

> serde_json::Value power_supply_impact_bottom_up_v1_component_power_supply_post(verbose, power_supply)
Power Supply Impact Bottom Up

# ✔️Power supply impacts from configuration ### 💡 Smart complete All missing data are retrieve with the closest available values. If no data are available default maximizing data are used  ### 👄 Verbose If set at true, shows the the values used for each attribute*Components have no units since they represent a single instance of a component.* ### 🧮 Measure <h3>psu<sub>manuf<sub><em>criteria</em></sub></sub> = psu<sub>unit<sub>weight</sub></sub> x psu<sub>manuf_weight<sub><em>criteria</em></sub></sub></h3> 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verbose** | Option<**bool**> |  |  |[default to true]
**power_supply** | Option<[**PowerSupply**](PowerSupply.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ram_impact_bottom_up_v1_component_ram_post

> serde_json::Value ram_impact_bottom_up_v1_component_ram_post(verbose, ram)
Ram Impact Bottom Up

# ✔️RAM impacts from configuration ### 💡 Smart complete All missing data are retrieve with the closest available values. If no data are available default maximizing data are used  ### 👄 Verbose If set at true, shows the the values used for each attribute*Components have no units since they represent a single instance of a component.* ### 🧮 Measure <h3>ram<sub>manuf<sub><em>criteria</em></sub></sub> =( ram<sub>size</sub> / ram<sub>density</sub> ) x ram<sub>manuf_die<sub><em>criteria</em></sub></sub> + ram<sub>manuf_base<sub><em>criteria</em></sub></sub> </h3> 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**verbose** | Option<**bool**> |  |  |[default to true]
**ram** | Option<[**Ram**](Ram.md)> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

