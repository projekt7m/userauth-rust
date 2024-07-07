# \ServiceApi

All URIs are relative to *https://login.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_services_superadmin**](ServiceApi.md#delete_services_superadmin) | **DELETE** /services/{id} | Deletes a service
[**get_services**](ServiceApi.md#get_services) | **GET** /services | Get the list of all known services of P7M
[**post_services_superadmin**](ServiceApi.md#post_services_superadmin) | **POST** /services | Create a new service



## delete_services_superadmin

> delete_services_superadmin(id)
Deletes a service

Deletes a service  **Note:** this can only been accessed by SUPERADMIN users (aka P7M itself)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the service | [required] |

### Return type

 (empty response body)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_services

> crate::models::ServiceData get_services()
Get the list of all known services of P7M

Get the list of all known services of P7M

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ServiceData**](ServiceData.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_services_superadmin

> crate::models::Service post_services_superadmin(new_service)
Create a new service

Create a new service  **Note:** This can only been accessed by SUPERADMIN users (aka P7M itself)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_service** | [**NewService**](NewService.md) | The service to be created | [required] |

### Return type

[**crate::models::Service**](Service.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

