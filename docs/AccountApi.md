# \AccountApi

All URIs are relative to *https://login.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_account_by_id**](AccountApi.md#delete_account_by_id) | **DELETE** /accounts/{id} | Delete a user account
[**get_account_by_id**](AccountApi.md#get_account_by_id) | **GET** /accounts/{id} | Get an account, that is known by its ID (UUID)
[**get_accounts**](AccountApi.md#get_accounts) | **GET** /accounts | Get the list of all accounts of the tenant
[**post_accounts**](AccountApi.md#post_accounts) | **POST** /accounts | 
[**put_account_by_id**](AccountApi.md#put_account_by_id) | **PUT** /accounts/{id} | Update an existing account



## delete_account_by_id

> delete_account_by_id(id)
Delete a user account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | ID of the account | [required] |

### Return type

 (empty response body)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_account_by_id

> models::Account get_account_by_id(id)
Get an account, that is known by its ID (UUID)

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | ID of the account | [required] |

### Return type

[**models::Account**](Account.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_accounts

> models::ListWrapperAccount get_accounts(changed_since)
Get the list of all accounts of the tenant

An account represents an authentication identity

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**changed_since** | Option<**String**> | only return accounts that have been changed since this datetime |  |

### Return type

[**models::ListWrapperAccount**](ListWrapper_Account.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_accounts

> models::Account post_accounts(new_account)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_account** | [**NewAccount**](NewAccount.md) | The account to be created | [required] |

### Return type

[**models::Account**](Account.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_account_by_id

> models::Account put_account_by_id(id, new_account)
Update an existing account

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **uuid::Uuid** | ID of the account | [required] |
**new_account** | [**NewAccount**](NewAccount.md) | Account to update | [required] |

### Return type

[**models::Account**](Account.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

