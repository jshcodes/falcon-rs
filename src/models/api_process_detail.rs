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
pub struct ApiProcessDetail {
    #[serde(rename = "command_line")]
    pub command_line: String,
    #[serde(rename = "device_id")]
    pub device_id: String,
    #[serde(rename = "file_name")]
    pub file_name: String,
    #[serde(rename = "process_id")]
    pub process_id: String,
    #[serde(rename = "process_id_local")]
    pub process_id_local: String,
    #[serde(rename = "start_timestamp")]
    pub start_timestamp: String,
    #[serde(rename = "start_timestamp_raw")]
    pub start_timestamp_raw: String,
    #[serde(rename = "stop_timestamp")]
    pub stop_timestamp: String,
    #[serde(rename = "stop_timestamp_raw")]
    pub stop_timestamp_raw: String,
}

impl ApiProcessDetail {
    pub fn new(command_line: String, device_id: String, file_name: String, process_id: String, process_id_local: String, start_timestamp: String, start_timestamp_raw: String, stop_timestamp: String, stop_timestamp_raw: String) -> ApiProcessDetail {
        ApiProcessDetail {
            command_line,
            device_id,
            file_name,
            process_id,
            process_id_local,
            start_timestamp,
            start_timestamp_raw,
            stop_timestamp,
            stop_timestamp_raw,
        }
    }
}


