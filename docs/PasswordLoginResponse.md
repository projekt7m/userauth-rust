# PasswordLoginResponse

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**success** | **bool** |  | 
**jwt** | Option<**String**> |  | [optional]
**account_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**realname** | Option<**String**> |  | [optional]
**tenant_id** | Option<[**uuid::Uuid**](uuid::Uuid.md)> |  | [optional]
**user_type** | Option<[**models::UserType**](UserType.md)> |  | [optional]
**expiration** | Option<**String**> |  | [optional]
**accessible_tenants** | [**Vec<models::Tenant>**](Tenant.md) |  | 

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


