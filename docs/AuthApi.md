# \AuthApi

All URIs are relative to *https://login.p7m.de/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**login_authorize_post**](AuthApi.md#login_authorize_post) | **POST** /login/authorize | 
[**login_post**](AuthApi.md#login_post) | **POST** /login | 



## login_authorize_post

> crate::models::AuthorizationResponse login_authorize_post(authorization_request)


# Authorize access to a given tenant and get JWT for that access  After the user has been authenticated with the a request to `/login` he got a JWT that is authorized to access the tenant the user belongs to. To access data of other tenants that are also accessible by the authenticated user, a request to this endpoint is required giving the ID of the tenant, that should get accessed. If the user has access to the data of the requested tenant, this endpoint will return another JWT, that can be used to access the data of the desired tenant. 

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**authorization_request** | [**AuthorizationRequest**](AuthorizationRequest.md) | which authorization is requested | [required] |

### Return type

[**crate::models::AuthorizationResponse**](AuthorizationResponse.md)

### Authorization

[Bearer](../README.md#Bearer)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## login_post

> crate::models::PasswordLoginResponse login_post(password_login_attempt)


# Authenticate (= proof who you are) to the system using a username and password  This is the first step for interacting with most of the functionality of P7M: the person interacting with the APIs has to authenticate to the system, so that the system knows who is accessing the system.  After authenticating, you automatically get a JWT that is authorized (= granted access) to work within the tenant, the authenticated user belongs to. If you want to access data of a different tenant, that is also accessible by the user, you have to get another JWT that is authorized (= granted access) to the other tenant. This can be done by following this request with an [authorization request](#/Auth/post_login_authorize) (see `/login/authorize`). 

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

