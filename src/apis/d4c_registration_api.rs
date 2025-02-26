/*
 * CrowdStrike API Specification
 *
 * Use this API specification as a reference for the API endpoints you can use to interact with your Falcon environment. These endpoints support authentication via OAuth2 and interact with detections and network containment. For detailed usage guides and more information about API endpoints that don't yet support OAuth2, see our [documentation inside the Falcon console](https://falcon.crowdstrike.com/support/documentation). To use the APIs described below, combine the base URL with the path shown for each API endpoint. For commercial cloud customers, your base URL is `https://api.crowdstrike.com`. Each API endpoint requires authorization via an OAuth2 token. Your first API request should retrieve an OAuth2 token using the `oauth2/token` endpoint, such as `https://api.crowdstrike.com/oauth2/token`. For subsequent requests, include the OAuth2 token in an HTTP authorization header. Tokens expire after 30 minutes, after which you should make a new token request to continue making API requests.
 *
 * The version of the OpenAPI document: 2021-10-05T19:33:53Z
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`create_cspm_azure_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCspmAzureAccountError {
    Status400(crate::models::RegistrationAzureAccountResponseV1),
    Status403(crate::models::MsaReplyMetaOnly),
    Status429(crate::models::MsaReplyMetaOnly),
    Status500(crate::models::RegistrationAzureAccountResponseV1),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_cspmgcp_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCspmgcpAccountError {
    Status400(crate::models::RegistrationGcpAccountResponseV1),
    Status403(crate::models::MsaReplyMetaOnly),
    Status429(crate::models::MsaReplyMetaOnly),
    Status500(crate::models::RegistrationGcpAccountResponseV1),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_cspm_azure_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCspmAzureAccountError {
    Status400(crate::models::RegistrationAzureAccountResponseV1),
    Status403(crate::models::MsaReplyMetaOnly),
    Status429(crate::models::MsaReplyMetaOnly),
    Status500(crate::models::RegistrationAzureAccountResponseV1),
    DefaultResponse(crate::models::RegistrationAzureAccountResponseV1),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_cspm_azure_user_scripts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCspmAzureUserScriptsError {
    Status400(crate::models::RegistrationAzureProvisionGetUserScriptResponseV1),
    Status403(crate::models::MsaReplyMetaOnly),
    Status429(crate::models::MsaReplyMetaOnly),
    Status500(crate::models::RegistrationAzureProvisionGetUserScriptResponseV1),
    DefaultResponse(crate::models::RegistrationAzureProvisionGetUserScriptResponseV1),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_cspm_azure_user_scripts_attachment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCspmAzureUserScriptsAttachmentError {
    Status400(crate::models::RegistrationAzureProvisionGetUserScriptResponseV1),
    Status403(crate::models::MsaReplyMetaOnly),
    Status429(crate::models::MsaReplyMetaOnly),
    Status500(crate::models::RegistrationAzureProvisionGetUserScriptResponseV1),
    DefaultResponse(crate::models::RegistrationAzureProvisionGetUserScriptResponseV1),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_cspmcgp_account`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCspmcgpAccountError {
    Status400(crate::models::RegistrationGcpAccountResponseV1),
    Status403(crate::models::MsaReplyMetaOnly),
    Status429(crate::models::MsaReplyMetaOnly),
    Status500(crate::models::RegistrationGcpAccountResponseV1),
    DefaultResponse(crate::models::RegistrationGcpAccountResponseV1),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_cspmgcp_user_scripts`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCspmgcpUserScriptsError {
    Status400(crate::models::RegistrationGcpProvisionGetUserScriptResponseV1),
    Status403(crate::models::MsaReplyMetaOnly),
    Status429(crate::models::MsaReplyMetaOnly),
    Status500(crate::models::RegistrationGcpProvisionGetUserScriptResponseV1),
    DefaultResponse(crate::models::RegistrationGcpProvisionGetUserScriptResponseV1),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_cspmgcp_user_scripts_attachment`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCspmgcpUserScriptsAttachmentError {
    Status400(crate::models::RegistrationGcpProvisionGetUserScriptResponseV1),
    Status403(crate::models::MsaReplyMetaOnly),
    Status429(crate::models::MsaReplyMetaOnly),
    Status500(crate::models::RegistrationGcpProvisionGetUserScriptResponseV1),
    DefaultResponse(crate::models::RegistrationGcpProvisionGetUserScriptResponseV1),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_cspm_azure_account_client_id`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateCspmAzureAccountClientIdError {
    Status400(crate::models::RegistrationAzureServicePrincipalResponseV1),
    Status403(crate::models::MsaReplyMetaOnly),
    Status429(crate::models::MsaReplyMetaOnly),
    Status500(crate::models::RegistrationAzureServicePrincipalResponseV1),
    UnknownValue(serde_json::Value),
}


pub async fn create_cspm_azure_account(configuration: &configuration::Configuration, body: crate::models::RegistrationAzureAccountCreateRequestExternalV1) -> Result<crate::models::RegistrationAzureAccountResponseV1, Error<CreateCspmAzureAccountError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/cloud-connect-azure/entities/account/v1", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateCspmAzureAccountError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn create_cspmgcp_account(configuration: &configuration::Configuration, body: crate::models::RegistrationGcpAccountCreateRequestExtV1) -> Result<crate::models::RegistrationGcpAccountResponseV1, Error<CreateCspmgcpAccountError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/cloud-connect-gcp/entities/account/v1", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&body);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateCspmgcpAccountError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_cspm_azure_account(configuration: &configuration::Configuration, ids: Option<Vec<String>>, scan_type: Option<&str>) -> Result<crate::models::RegistrationAzureAccountResponseV1, Error<GetCspmAzureAccountError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/cloud-connect-azure/entities/account/v1", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = ids {
        local_var_req_builder = local_var_req_builder.query(&[("ids", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]);
    }
    if let Some(ref local_var_str) = scan_type {
        local_var_req_builder = local_var_req_builder.query(&[("scan-type", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetCspmAzureAccountError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_cspm_azure_user_scripts(configuration: &configuration::Configuration, ) -> Result<crate::models::RegistrationAzureProvisionGetUserScriptResponseV1, Error<GetCspmAzureUserScriptsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/cloud-connect-azure/entities/user-scripts/v1", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetCspmAzureUserScriptsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_cspm_azure_user_scripts_attachment(configuration: &configuration::Configuration, ) -> Result<crate::models::RegistrationAzureProvisionGetUserScriptResponseV1, Error<GetCspmAzureUserScriptsAttachmentError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/cloud-connect-azure/entities/user-scripts-download/v1", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetCspmAzureUserScriptsAttachmentError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_cspmcgp_account(configuration: &configuration::Configuration, scan_type: Option<&str>, ids: Option<Vec<String>>) -> Result<crate::models::RegistrationGcpAccountResponseV1, Error<GetCspmcgpAccountError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/cloud-connect-gcp/entities/account/v1", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = scan_type {
        local_var_req_builder = local_var_req_builder.query(&[("scan-type", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = ids {
        local_var_req_builder = local_var_req_builder.query(&[("ids", &local_var_str.into_iter().map(|p| p.to_string()).collect::<Vec<String>>().join(",").to_string())]);
    }
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetCspmcgpAccountError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_cspmgcp_user_scripts(configuration: &configuration::Configuration, ) -> Result<crate::models::RegistrationGcpProvisionGetUserScriptResponseV1, Error<GetCspmgcpUserScriptsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/cloud-connect-gcp/entities/user-scripts/v1", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetCspmgcpUserScriptsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn get_cspmgcp_user_scripts_attachment(configuration: &configuration::Configuration, ) -> Result<crate::models::RegistrationGcpProvisionGetUserScriptResponseV1, Error<GetCspmgcpUserScriptsAttachmentError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/cloud-connect-gcp/entities/user-scripts-download/v1", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetCspmgcpUserScriptsAttachmentError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn update_cspm_azure_account_client_id(configuration: &configuration::Configuration, id: &str) -> Result<crate::models::RegistrationAzureServicePrincipalResponseV1, Error<UpdateCspmAzureAccountClientIdError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/cloud-connect-azure/entities/client-id/v1", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("id", &id.to_string())]);
    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateCspmAzureAccountClientIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

