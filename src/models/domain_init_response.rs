/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and more information about API endpoints that don't yet support OAuth2, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation). To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`. Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: 2021-10-05T19:33:53Z
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct DomainInitResponse {
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "existing_aid_sessions")]
    pub existing_aid_sessions: i32,
    #[serde(rename = "offline_queued")]
    pub offline_queued: bool,
    #[serde(rename = "previous_commands", skip_serializing_if = "Option::is_none")]
    pub previous_commands: Option<Vec<String>>,
    #[serde(rename = "pwd", skip_serializing_if = "Option::is_none")]
    pub pwd: Option<String>,
    #[serde(rename = "scripts")]
    pub scripts: Vec<crate::models::DomainScriptHelp>,
    #[serde(rename = "session_id")]
    pub session_id: String,
}

impl DomainInitResponse {
    pub fn new(created_at: String, existing_aid_sessions: i32, offline_queued: bool, scripts: Vec<crate::models::DomainScriptHelp>, session_id: String) -> DomainInitResponse {
        DomainInitResponse {
            created_at,
            existing_aid_sessions,
            offline_queued,
            previous_commands: None,
            pwd: None,
            scripts,
            session_id,
        }
    }
}


