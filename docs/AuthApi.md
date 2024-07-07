# \AuthApi

All URIs are relative to *https://login.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**post_login**](AuthApi.md#post_login) | **POST** /login | Authenticate (= proof who you are) to the system using a username and password
[**post_login_authorize**](AuthApi.md#post_login_authorize) | **POST** /login/authorize | Authorize access to a given tenant and get JWT for that access



## post_login

> crate::models::PasswordLoginResponse post_login(password_login_attempt)
Authenticate (= proof who you are) to the system using a username and password

Authenticate (= proof who you are) to the system using a username and password  This is the first step for interacting with most of the functionality of P7M: the person interacting with the APIs has to authenticate to the system, so that the system knows who is accessing the system.  After authenticating, you automatically get a JWT that is authorized (= granted access) to work within the tenant, the authenticated user belongs to. If you want to access data of a different tenant, that is also accessible by the user, you have to get another JWT that is authorized (= granted access) to the other tenant. This can be done by following this request with an authorization request (see `/login/authorize`).  **Note:** This endpoint is deprecated, use OAuth 2.0 login instead.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**password_login_attempt** | [**PasswordLoginAttempt**](PasswordLoginAttempt.md) | data for the login attempt | [required] |

### Return type

[**crate::models::PasswordLoginResponse**](PasswordLoginResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_login_authorize

> crate::models::AuthorizationResponse post_login_authorize(authorization_request)
Authorize access to a given tenant and get JWT for that access

Authorize access to a given tenant and get JWT for that access  After the user has been authenticated with a request to `/login` he got a JWT that is authorized to access the tenant the user belongs to. To access data of other tenants that are also accessible by the authenticated user, a request to this endpoint is required giving the ID of the tenant, that should get accessed. If the user has access to the data of the requested tenant, this endpoint will return another JWT, that can be used to access the data of the desired tenant.  **Note**: This endpoint is deprecated, use OAuth 2.0 login instead. While loggin in the user can select the tenant it wants to work within.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization_request** | [**AuthorizationRequest**](AuthorizationRequest.md) | which authorization is requested | [required] |

### Return type

[**crate::models::AuthorizationResponse**](AuthorizationResponse.md)

### Authorization

[oauth](../README.md#oauth)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

