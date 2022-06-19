# \AccountApi

All URIs are relative to *https://login.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**accounts_get**](AccountApi.md#accounts_get) | **GET** /accounts | 
[**accounts_id_delete**](AccountApi.md#accounts_id_delete) | **DELETE** /accounts/{id} | 
[**accounts_id_get**](AccountApi.md#accounts_id_get) | **GET** /accounts/{id} | 
[**accounts_id_put**](AccountApi.md#accounts_id_put) | **PUT** /accounts/{id} | 
[**accounts_post**](AccountApi.md#accounts_post) | **POST** /accounts | 



## accounts_get

> crate::models::AccountData accounts_get()


an account represents an authentication identity

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::AccountData**](AccountData.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accounts_id_delete

> accounts_id_delete(id)


deletes the given account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | account id | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accounts_id_get

> crate::models::Account accounts_id_get(id)


Returns the given account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | account id | [required] |

### Return type

[**crate::models::Account**](Account.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accounts_id_put

> crate::models::Account accounts_id_put(id, account)


updates the given account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | account id | [required] |
**account** | [**Account**](Account.md) | the updated account | [required] |

### Return type

[**crate::models::Account**](Account.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## accounts_post

> crate::models::Account accounts_post(new_account)


creates a new account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_account** | [**NewAccount**](NewAccount.md) | the account to be created | [required] |

### Return type

[**crate::models::Account**](Account.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

