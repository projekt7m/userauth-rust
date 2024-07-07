# \TenantApi

All URIs are relative to *https://login.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_tenant_by_id**](TenantApi.md#delete_tenant_by_id) | **DELETE** /tenants/{id} | Delete a tenant specified by the tenant's ID
[**get_tenant_by_id**](TenantApi.md#get_tenant_by_id) | **GET** /tenants/{id} | Request a the tenant identified by its ID
[**get_tenants**](TenantApi.md#get_tenants) | **GET** /tenants | Get the list of tenants
[**post_tenants**](TenantApi.md#post_tenants) | **POST** /tenants | 
[**put_tenant_by_id**](TenantApi.md#put_tenant_by_id) | **PUT** /tenants/{id} | Update an existing tenant



## delete_tenant_by_id

> delete_tenant_by_id(id)
Delete a tenant specified by the tenant's ID

Delete a tenant specified by the tenant's ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the tenant | [required] |

### Return type

 (empty response body)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: Not defined

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tenant_by_id

> crate::models::Tenant get_tenant_by_id(id)
Request a the tenant identified by its ID

Request a the tenant identified by its ID

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the tenant | [required] |

### Return type

[**crate::models::Tenant**](Tenant.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_tenants

> crate::models::TenantData get_tenants()
Get the list of tenants

Get the list of tenants  A tenant is typically a company or surgery that uses services of P7M.  Data within the services of P7M arescoped and isolated between tenants. A tenant represents the boundaries between the data of different clients. Data within P7M is stored tagged with the tenant_id to identify the owning tenant of the datum.

### Parameters

This endpoint does not need any parameter.

### Return type

[**crate::models::TenantData**](TenantData.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_tenants

> crate::models::Tenant post_tenants(new_tenant)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**new_tenant** | [**NewTenant**](NewTenant.md) | The tenant to be created | [required] |

### Return type

[**crate::models::Tenant**](Tenant.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_tenant_by_id

> crate::models::Tenant put_tenant_by_id(id, new_tenant)
Update an existing tenant

Update an existing tenant

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**id** | **String** | ID of the tenant | [required] |
**new_tenant** | [**NewTenant**](NewTenant.md) | The updated tenant | [required] |

### Return type

[**crate::models::Tenant**](Tenant.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

