# \TenantApi

All URIs are relative to *https://login.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**tenants_get**](TenantApi.md#tenants_get) | **GET** /tenants | 
[**tenants_id_delete**](TenantApi.md#tenants_id_delete) | **DELETE** /tenants/{id} | 
[**tenants_id_get**](TenantApi.md#tenants_id_get) | **GET** /tenants/{id} | 
[**tenants_id_put**](TenantApi.md#tenants_id_put) | **PUT** /tenants/{id} | 
[**tenants_post**](TenantApi.md#tenants_post) | **POST** /tenants | 



## tenants_get

> crate::models::TenantData tenants_get()


a tenant is typically a company or surgery that has book services of P7M  Data within the services of P7M are scoped and isolated between tenants. A tenant represents the boundaries between the data of different clients. Data within P7M is stored tagged with the tenant_id to identify the owning tenant of the datum. 

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::TenantData**](TenantData.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenants_id_delete

> tenants_id_delete(id)


deletes the specified tenant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | tenant id | [required] |

### Return type

 (empty response body)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenants_id_get

> crate::models::Tenant tenants_id_get(id)


Returns the specified tenant 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | tenant id | [required] |

### Return type

[**crate::models::Tenant**](Tenant.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenants_id_put

> crate::models::Tenant tenants_id_put(id, tenant)


updates the specified tenant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | tenant id | [required] |
**tenant** | [**Tenant**](Tenant.md) | the updated tenant | [required] |

### Return type

[**crate::models::Tenant**](Tenant.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## tenants_post

> crate::models::Tenant tenants_post(new_tenant)


creates a new tenant  This endpoint is only usable with special administrative permissions and cannot be used by normal users of this API. A new tenant object is created for P7M clients at setup time by P7M. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_tenant** | [**NewTenant**](NewTenant.md) | the tenant to be created | [required] |

### Return type

[**crate::models::Tenant**](Tenant.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

