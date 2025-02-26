/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and more information about API endpoints that don't yet support OAuth2, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation). To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`. Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: 2021-10-05T19:33:53Z
 * 
 * Generated by: https://openapi-generator.tech
 */

/// RequestsDeviceControlPolicySettingsV1 : A specific setting to update



#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct RequestsDeviceControlPolicySettingsV1 {
    /// Settings that apply to a USB Class
    #[serde(rename = "classes")]
    pub classes: Vec<crate::models::RequestsDeviceControlPolicyClassSettingsV1>,
    /// Does the end user receives a notification when the policy is violated
    #[serde(rename = "end_user_notification")]
    pub end_user_notification: EndUserNotification,
    /// How is this policy enforced
    #[serde(rename = "enforcement_mode")]
    pub enforcement_mode: String,
    /// The id of the setting to update
    #[serde(rename = "id")]
    pub id: String,
}

impl RequestsDeviceControlPolicySettingsV1 {
    /// A specific setting to update
    pub fn new(classes: Vec<crate::models::RequestsDeviceControlPolicyClassSettingsV1>, end_user_notification: EndUserNotification, enforcement_mode: String, id: String) -> RequestsDeviceControlPolicySettingsV1 {
        RequestsDeviceControlPolicySettingsV1 {
            classes,
            end_user_notification,
            enforcement_mode,
            id,
        }
    }
}

/// Does the end user receives a notification when the policy is violated
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum EndUserNotification {
    #[serde(rename = "TRUE")]
    _TRUE,
    #[serde(rename = "FALSE")]
    _FALSE,
}

