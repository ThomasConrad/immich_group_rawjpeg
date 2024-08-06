# \ServerApi

All URIs are relative to *https://raw.githubusercontent.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_server_license**](ServerApi.md#delete_server_license) | **DELETE** /server/license | 
[**get_server_license**](ServerApi.md#get_server_license) | **GET** /server/license | 
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

