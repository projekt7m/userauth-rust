# \ClientApi

All URIs are relative to *https://login.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_clients_by_id_superadmin**](ClientApi.md#delete_clients_by_id_superadmin) | **DELETE** /clients/{id} | Deletes an OAuth client
[**get_clients_by_id_superadmin**](ClientApi.md#get_clients_by_id_superadmin) | **GET** /clients/{id} | Requests a single OAuth client by its ID
[**get_clients_superadmin**](ClientApi.md#get_clients_superadmin) | **GET** /clients | Gets the list of all registered OAuth clients
[**post_clients_superadmin**](ClientApi.md#post_clients_superadmin) | **POST** /clients | Create a new OAuth client
[**put_clients_by_id_superadmin**](ClientApi.md#put_clients_by_id_superadmin) | **PUT** /clients/{id} | Updates an OAuth client



## delete_clients_by_id_superadmin

> delete_clients_by_id_superadmin(id)
Deletes an OAuth client

Deletes an OAuth client

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the client | [required] |

### Return type

 (empty response body)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_clients_by_id_superadmin

> crate::models::Client get_clients_by_id_superadmin(id)
Requests a single OAuth client by its ID

Requests a single OAuth client by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the client | [required] |

### Return type

[**crate::models::Client**](Client.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_clients_superadmin

> crate::models::ClientData get_clients_superadmin()
Gets the list of all registered OAuth clients

Gets the list of all registered OAuth clients  This endpoint is only available to SuperAdmins.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::ClientData**](ClientData.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_clients_superadmin

> crate::models::Client post_clients_superadmin(new_client)
Create a new OAuth client

Create a new OAuth client  This endpoint is only available to SuperAdmin users.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_client** | [**NewClient**](NewClient.md) | The client to be created | [required] |

### Return type

[**crate::models::Client**](Client.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_clients_by_id_superadmin

> crate::models::Client put_clients_by_id_superadmin(id, client_update)
Updates an OAuth client

Updates an OAuth client

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the client | [required] |
**client_update** | [**ClientUpdate**](ClientUpdate.md) | The updated client | [required] |

### Return type

[**crate::models::Client**](Client.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

