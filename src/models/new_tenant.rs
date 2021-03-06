/*
 * User and Authentication Backend
 *
 * # API for authentication and managing user accounts  This is the API of the service at P7M that manages tenants, accounts and authentication. It is the basis of many services of P7M.  For most endpoints, the caller has to be authenticated with the system and provide a JWT token in the Authorization header of the HTTP request. If your interacting with this API using the Swagger interface, you need to set the JWT token by clicking on the **Authorize** button on the right side of the header. As the value don't forget that the Authorization header starts with the fixed value `Bearer` followed by a space followed by the actual JWT token value.  If anything is unclear or you found a bug in the documentation, please contact <tech@p7m.de>. 
 *
 * The version of the OpenAPI document: 0.11.0
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct NewTenant {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "realm")]
    pub realm: String,
    #[serde(rename = "timezone")]
    pub timezone: String,
    #[serde(rename = "parentId", skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
}

impl NewTenant {
    pub fn new(name: String, realm: String, timezone: String) -> NewTenant {
        NewTenant {
            name,
            realm,
            timezone,
            parent_id: None,
        }
    }
}


