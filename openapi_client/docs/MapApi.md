# \MapApi

All URIs are relative to *https://raw.githubusercontent.com/api*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_map_markers**](MapApi.md#get_map_markers) | **GET** /map/markers | 
[**get_map_style**](MapApi.md#get_map_style) | **GET** /map/style.json | 
[**reverse_geocode**](MapApi.md#reverse_geocode) | **GET** /map/reverse-geocode | 



## get_map_markers

> Vec<models::MapMarkerResponseDto> get_map_markers(file_created_after, file_created_before, is_archived, is_favorite, with_partners, with_shared_albums)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**file_created_after** | Option<**String**> |  |  |
**file_created_before** | Option<**String**> |  |  |
**is_archived** | Option<**bool**> |  |  |
**is_favorite** | Option<**bool**> |  |  |
**with_partners** | Option<**bool**> |  |  |
**with_shared_albums** | Option<**bool**> |  |  |

### Return type

[**Vec<models::MapMarkerResponseDto>**](MapMarkerResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_map_style

> serde_json::Value get_map_style(theme, key)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**theme** | [**MapTheme**](.md) |  | [required] |
**key** | Option<**String**> |  |  |

### Return type

[**serde_json::Value**](serde_json::Value.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## reverse_geocode

> Vec<models::MapReverseGeocodeResponseDto> reverse_geocode(lat, lon)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**lat** | **f64** |  | [required] |
**lon** | **f64** |  | [required] |

### Return type

[**Vec<models::MapReverseGeocodeResponseDto>**](MapReverseGeocodeResponseDto.md)

### Authorization

[cookie](../README.md#cookie), [api_key](../README.md#api_key), [bearer](../README.md#bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

