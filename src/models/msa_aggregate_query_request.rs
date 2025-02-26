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
pub struct MsaAggregateQueryRequest {
    #[serde(rename = "date_ranges")]
    pub date_ranges: Vec<crate::models::MsaDateRangeSpec>,
    #[serde(rename = "field")]
    pub field: String,
    #[serde(rename = "filter")]
    pub filter: String,
    #[serde(rename = "interval")]
    pub interval: String,
    #[serde(rename = "min_doc_count")]
    pub min_doc_count: i64,
    #[serde(rename = "missing")]
    pub missing: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "q")]
    pub q: String,
    #[serde(rename = "ranges")]
    pub ranges: Vec<crate::models::MsaRangeSpec>,
    #[serde(rename = "size")]
    pub size: i32,
    #[serde(rename = "sort")]
    pub sort: String,
    #[serde(rename = "sub_aggregates")]
    pub sub_aggregates: Vec<crate::models::MsaAggregateQueryRequest>,
    #[serde(rename = "time_zone")]
    pub time_zone: String,
    #[serde(rename = "type")]
    pub _type: String,
}

impl MsaAggregateQueryRequest {
    pub fn new(date_ranges: Vec<crate::models::MsaDateRangeSpec>, field: String, filter: String, interval: String, min_doc_count: i64, missing: String, name: String, q: String, ranges: Vec<crate::models::MsaRangeSpec>, size: i32, sort: String, sub_aggregates: Vec<crate::models::MsaAggregateQueryRequest>, time_zone: String, _type: String) -> MsaAggregateQueryRequest {
        MsaAggregateQueryRequest {
            date_ranges,
            field,
            filter,
            interval,
            min_doc_count,
            missing,
            name,
            q,
            ranges,
            size,
            sort,
            sub_aggregates,
            time_zone,
            _type,
        }
    }
}


