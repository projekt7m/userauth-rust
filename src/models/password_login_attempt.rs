/*
 * User and Authentication Backend
 *
 * # API for authentication and managing user accounts  This is the API of the service at P7M that manages tenants, accounts and authentication. It is the basis of many services of P7M.  The caller has to be authenticated with the system and provide a JWT token in the Authorization header of the HTTP request. When using the API you typically get this token by authenticating first with OAuth 2.0.  When you are trying this API using the Swagger interface, you need to click the `Authorize` button and then again the Authorize button in the pop-up that gets opened.
 *
 * The version of the OpenAPI document: 0.13.0
 * Contact: tech@p7m.de
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct PasswordLoginAttempt {
    #[serde(rename = "username")]
    pub username: String,
    #[serde(rename = "realm")]
    pub realm: String,
    #[serde(rename = "password")]
    pub password: String,
}

impl PasswordLoginAttempt {
    pub fn new(username: String, realm: String, password: String) -> PasswordLoginAttempt {
        PasswordLoginAttempt {
            username,
            realm,
            password,
        }
    }
}


