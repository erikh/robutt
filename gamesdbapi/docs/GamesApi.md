# \GamesApi

All URIs are relative to *https://api.thegamesdb.net*

Method | HTTP request | Description
------------- | ------------- | -------------
[**games_by_game_id**](GamesApi.md#games_by_game_id) | **get** /v1/Games/ByGameID | Fetch game(s) by id
[**games_by_game_name**](GamesApi.md#games_by_game_name) | **get** /v1.1/Games/ByGameName | Fetch game(s) by name
[**games_by_game_name_v1**](GamesApi.md#games_by_game_name_v1) | **get** /v1/Games/ByGameName | Fetch game(s) by name
[**games_by_platform_id**](GamesApi.md#games_by_platform_id) | **get** /v1/Games/ByPlatformID | Fetch game(s) by platform id
[**games_images**](GamesApi.md#games_images) | **get** /v1/Games/Images | Fetch game(s) images by game(s) id
[**games_updates**](GamesApi.md#games_updates) | **get** /v1/Games/Updates | Fetch games update



## games_by_game_id

> crate::models::GamesByGameId games_by_game_id(apikey, id, fields, include, page)
Fetch game(s) by id

can request additional information can be requestes through `fields` and `include` params

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apikey** | **String** | (Required) | [required] |
**id** | **String** | (Required) - supports `,` delimited list | [required] |
**fields** | Option<**String**> | (Optional) - valid `,` delimited options: `players`, `publishers`, `genres`, `overview`, `last_updated`, `rating`, `platform`, `coop`, `youtube`, `os`, `processor`, `ram`, `hdd`, `video`, `sound`, `alternates` |  |
**include** | Option<**String**> | (Optional) - valid `,` delimited options: `boxart`, `platform` |  |
**page** | Option<**i32**> | (Optional) - results page offset to return |  |

### Return type

[**crate::models::GamesByGameId**](GamesByGameID.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## games_by_game_name

> crate::models::GamesByGameId games_by_game_name(apikey, name, fields, filter_platform, include, page)
Fetch game(s) by name

can request additional information can be requestes through `fields` and `include` params

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apikey** | **String** | (Required) | [required] |
**name** | **String** | (Required) - Search term | [required] |
**fields** | Option<**String**> | (Optional) - valid `,` delimited options: `players`, `publishers`, `genres`, `overview`, `last_updated`, `rating`, `platform`, `coop`, `youtube`, `os`, `processor`, `ram`, `hdd`, `video`, `sound`, `alternates` |  |
**filter_platform** | Option<**String**> | (Optional) - platform `id` can be obtain from the platforms api below, supports `,` delimited list |  |
**include** | Option<**String**> | (Optional) - valid `,` delimited options: `boxart`, `platform` |  |
**page** | Option<**i32**> | (Optional) - results page offset to return |  |

### Return type

[**crate::models::GamesByGameId**](GamesByGameID.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## games_by_game_name_v1

> crate::models::GamesByGameIdV1 games_by_game_name_v1(apikey, name, fields, filter_platform, include, page)
Fetch game(s) by name

can request additional information can be requestes through `fields` and `include` params

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apikey** | **String** | (Required) | [required] |
**name** | **String** | (Required) - Search term | [required] |
**fields** | Option<**String**> | (Optional) - valid `,` delimited options: `players`, `publishers`, `genres`, `overview`, `last_updated`, `rating`, `platform`, `coop`, `youtube`, `os`, `processor`, `ram`, `hdd`, `video`, `sound`, `alternates` |  |
**filter_platform** | Option<**String**> | (Optional) - platform `id` can be obtain from the platforms api below, supports `,` delimited list |  |
**include** | Option<**String**> | (Optional) - valid `,` delimited options: `boxart`, `platform` |  |
**page** | Option<**i32**> | (Optional) - results page offset to return |  |

### Return type

[**crate::models::GamesByGameIdV1**](GamesByGameID_v1.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## games_by_platform_id

> crate::models::GamesByGameId games_by_platform_id(apikey, id, fields, include, page)
Fetch game(s) by platform id

can request additional information can be requested through `fields` and `include` params

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apikey** | **String** | (Required) | [required] |
**id** | **String** | (Required) - platform `id` can be obtain from the platforms api below, supports `,` delimited list | [required] |
**fields** | Option<**String**> | (Optional) - valid `,` delimited options: `players`, `publishers`, `genres`, `overview`, `last_updated`, `rating`, `platform`, `coop`, `youtube`, `os`, `processor`, `ram`, `hdd`, `video`, `sound`, `alternates` |  |
**include** | Option<**String**> | (Optional) - valid `,` delimited options: `boxart`, `platform` |  |
**page** | Option<**i32**> | (Optional) - results page offset to return |  |

### Return type

[**crate::models::GamesByGameId**](GamesByGameID.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## games_images

> crate::models::GamesImages games_images(apikey, games_id, filter_type, page)
Fetch game(s) images by game(s) id

results can be filtered with `filter[type]` param

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apikey** | **String** | (Required) | [required] |
**games_id** | **String** | (Required) - game(s) `id` can be obtain from the above games api, supports `,` delimited list | [required] |
**filter_type** | Option<**String**> | (Optional) - valid `,` delimited options: `fanart`, `banner`, `boxart`, `screenshot`, `clearlogo`, `titlescreen` |  |
**page** | Option<**i32**> | (Optional) - results page offset to return |  |

### Return type

[**crate::models::GamesImages**](GamesImages.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## games_updates

> crate::models::GamesUpdates games_updates(apikey, last_edit_id, time, page)
Fetch games update

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**apikey** | **String** | (Required) | [required] |
**last_edit_id** | **i32** | (Required) | [required] |
**time** | Option<**i32**> | (Optional) |  |
**page** | Option<**i32**> | (Optional) - results page offset to return |  |

### Return type

[**crate::models::GamesUpdates**](GamesUpdates.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

