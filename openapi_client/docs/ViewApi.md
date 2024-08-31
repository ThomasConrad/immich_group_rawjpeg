# \ViewApi

All URIs are relative to *https://raw.githubusercontent.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_assets_by_original_path**](ViewApi.md#get_assets_by_original_path) | **GET** /view/folder | 
[**get_unique_original_paths**](ViewApi.md#get_unique_original_paths) | **GET** /view/folder/unique-paths | 



## get_assets_by_original_path

> Vec<models::AssetResponseDto> get_assets_by_original_path(path)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**path** | **String** |  | [required] |

### Return type

[**Vec<models::AssetResponseDto>**](AssetResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_unique_original_paths

> Vec<String> get_unique_original_paths()


### Parameters

This endpoint does not need any parameter.

### Return type

**Vec<String>**

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

