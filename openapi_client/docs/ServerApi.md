# \ServerApi

All URIs are relative to *https://raw.githubusercontent.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_server_license**](ServerApi.md#delete_server_license) | **DELETE** /server/license | 
[**get_about_info**](ServerApi.md#get_about_info) | **GET** /server/about | 
[**get_server_config**](ServerApi.md#get_server_config) | **GET** /server/config | 
[**get_server_features**](ServerApi.md#get_server_features) | **GET** /server/features | 
[**get_server_license**](ServerApi.md#get_server_license) | **GET** /server/license | 
[**get_server_statistics**](ServerApi.md#get_server_statistics) | **GET** /server/statistics | 
[**get_server_version**](ServerApi.md#get_server_version) | **GET** /server/version | 
[**get_storage**](ServerApi.md#get_storage) | **GET** /server/storage | 
[**get_supported_media_types**](ServerApi.md#get_supported_media_types) | **GET** /server/media-types | 
[**get_theme**](ServerApi.md#get_theme) | **GET** /server/theme | 
[**ping_server**](ServerApi.md#ping_server) | **GET** /server/ping | 
[**set_server_license**](ServerApi.md#set_server_license) | **PUT** /server/license | 



## delete_server_license

> delete_server_license()


### Parameters

This endpoint does not need any parameter.

### Return type

 (empty response body)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_about_info

> models::ServerAboutResponseDto get_about_info()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ServerAboutResponseDto**](ServerAboutResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_server_config

> models::ServerConfigDto get_server_config()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ServerConfigDto**](ServerConfigDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_server_features

> models::ServerFeaturesDto get_server_features()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ServerFeaturesDto**](ServerFeaturesDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_server_license

> models::LicenseResponseDto get_server_license()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::LicenseResponseDto**](LicenseResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_server_statistics

> models::ServerStatsResponseDto get_server_statistics()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ServerStatsResponseDto**](ServerStatsResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_server_version

> models::ServerVersionResponseDto get_server_version()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ServerVersionResponseDto**](ServerVersionResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_storage

> models::ServerStorageResponseDto get_storage()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ServerStorageResponseDto**](ServerStorageResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_supported_media_types

> models::ServerMediaTypesResponseDto get_supported_media_types()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ServerMediaTypesResponseDto**](ServerMediaTypesResponseDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_theme

> models::ServerThemeDto get_theme()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ServerThemeDto**](ServerThemeDto.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## ping_server

> models::ServerPingResponse ping_server()


### Parameters

This endpoint does not need any parameter.

### Return type

[**models::ServerPingResponse**](ServerPingResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## set_server_license

> models::LicenseResponseDto set_server_license(license_key_dto)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**license_key_dto** | [**LicenseKeyDto**](LicenseKeyDto.md) |  | [required] |

### Return type

[**models::LicenseResponseDto**](LicenseResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

