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
pub struct FalconxFileMetadata {
    #[serde(rename = "file_analysis", skip_serializing_if = "Option::is_none")]
    pub file_analysis: Option<Vec<String>>,
    #[serde(rename = "file_compositions", skip_serializing_if = "Option::is_none")]
    pub file_compositions: Option<Vec<String>>,
    #[serde(rename = "imported_objects", skip_serializing_if = "Option::is_none")]
    pub imported_objects: Option<Vec<String>>,
}

impl FalconxFileMetadata {
    pub fn new() -> FalconxFileMetadata {
        FalconxFileMetadata {
            file_analysis: None,
            file_compositions: None,
            imported_objects: None,
        }
    }
}


