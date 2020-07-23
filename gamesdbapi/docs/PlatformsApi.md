# \PlatformsApi

All URIs are relative to *https://api.thegamesdb.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**platforms**](PlatformsApi.md#platforms) | **get** /v1/Platforms | Fetch platforms list
[**platforms_by_platform_id**](PlatformsApi.md#platforms_by_platform_id) | **get** /v1/Platforms/ByPlatformID | Fetch platforms list by id
[**platforms_by_platform_name**](PlatformsApi.md#platforms_by_platform_name) | **get** /v1/Platforms/ByPlatformName | Fetch platforms by name
[**platforms_images**](PlatformsApi.md#platforms_images) | **get** /v1/Platforms/Images | Fetch platform(s) images by platform(s) id



## platforms

> crate::models::Platforms platforms(apikey, fields)
Fetch platforms list

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apikey** | **String** | (Required) | [required] |
**fields** | Option<**String**> | (Optional) - valid `,` delimited options: `icon`, `console`, `controller`, `developer`, `manufacturer`, `media`, `cpu`, `memory`, `graphics`, `sound`, `maxcontrollers`, `display`, `overview`, `youtube` |  |

### Return type

[**crate::models::Platforms**](Platforms.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## platforms_by_platform_id

> crate::models::PlatformsByPlatformId platforms_by_platform_id(apikey, id, fields)
Fetch platforms list by id

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apikey** | **String** | (Required) | [required] |
**id** | **i32** | (Required) - supports `,` delimited list | [required] |
**fields** | Option<**String**> | (Optional) - valid `,` delimited options: `icon`, `console`, `controller`, `developer`, `manufacturer`, `media`, `cpu`, `memory`, `graphics`, `sound`, `maxcontrollers`, `display`, `overview`, `youtube` |  |

### Return type

[**crate::models::PlatformsByPlatformId**](PlatformsByPlatformID.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## platforms_by_platform_name

> crate::models::PlatformsByPlatformName platforms_by_platform_name(apikey, name, fields)
Fetch platforms by name

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apikey** | **String** | (Required) | [required] |
**name** | **String** | (Required) | [required] |
**fields** | Option<**String**> | (Optional) - valid `,` delimited options: `icon`, `console`, `controller`, `developer`, `manufacturer`, `media`, `cpu`, `memory`, `graphics`, `sound`, `maxcontrollers`, `display`, `overview`, `youtube` |  |

### Return type

[**crate::models::PlatformsByPlatformName**](PlatformsByPlatformName.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## platforms_images

> crate::models::PlatformsImages platforms_images(apikey, platforms_id, filter_type, page)
Fetch platform(s) images by platform(s) id

results can be filtered with `filter[type]` param

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apikey** | **String** | (Required) | [required] |
**platforms_id** | **String** | (Required) - platform(s) `id` can be obtain from the above platforms api, supports `,` delimited list | [required] |
**filter_type** | Option<**String**> | (Optional) - valid `,` delimited options: `fanart`, `banner`, `boxart` |  |
**page** | Option<**i32**> | (Optional) - results page offset to return |  |

### Return type

[**crate::models::PlatformsImages**](PlatformsImages.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

