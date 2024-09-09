/*
 * Fastly API
 *
 * Via the Fastly API you can perform any of the operations that are possible within the management console,  including creating services, domains, and backends, configuring rules or uploading your own application code, as well as account operations such as user administration and billing reports. The API is organized into collections of endpoints that allow manipulation of objects related to Fastly services and accounts. For the most accurate and up-to-date API reference content, visit our [Developer Hub](https://www.fastly.com/documentation/reference/api/) 
 *
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};

/// struct for passing parameters to the method [`create_log_digocean`]
#[derive(Clone, Debug, Default)]
pub struct CreateLogDigoceanParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// The name for the real-time logging configuration.
    pub name: Option<String>,
    /// Where in the generated VCL the logging call should be placed. If not set, endpoints with `format_version` of 2 are placed in `vcl_log` and those with `format_version` of 1 are placed in `vcl_deliver`. 
    pub placement: Option<String>,
    /// The name of an existing condition in the configured endpoint, or leave blank to always execute.
    pub response_condition: Option<String>,
    /// A Fastly [log format string](https://docs.fastly.com/en/guides/custom-log-formats).
    pub format: Option<String>,
    /// The version of the custom logging format used for the configured endpoint. The logging call gets placed by default in `vcl_log` if `format_version` is set to `2` and in `vcl_deliver` if `format_version` is set to `1`. 
    pub format_version: Option<i32>,
    /// How the message should be formatted.
    pub message_type: Option<String>,
    /// A timestamp format
    pub timestamp_format: Option<String>,
    /// The codec used for compressing your logs. Valid values are `zstd`, `snappy`, and `gzip`. Specifying both `compression_codec` and `gzip_level` in the same API request will result in an error.
    pub compression_codec: Option<String>,
    /// How frequently log files are finalized so they can be available for reading (in seconds).
    pub period: Option<i32>,
    /// The level of gzip encoding when sending logs (default `0`, no compression). Specifying both `compression_codec` and `gzip_level` in the same API request will result in an error.
    pub gzip_level: Option<i32>,
    /// The name of the DigitalOcean Space.
    pub bucket_name: Option<String>,
    /// Your DigitalOcean Spaces account access key.
    pub access_key: Option<String>,
    /// Your DigitalOcean Spaces account secret key.
    pub secret_key: Option<String>,
    /// The domain of the DigitalOcean Spaces endpoint.
    pub domain: Option<String>,
    /// The path to upload logs to.
    pub path: Option<String>,
    /// A PGP public key that Fastly will use to encrypt your log files before writing them to disk.
    pub public_key: Option<String>
}

/// struct for passing parameters to the method [`delete_log_digocean`]
#[derive(Clone, Debug, Default)]
pub struct DeleteLogDigoceanParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// The name for the real-time logging configuration.
    pub logging_digitalocean_name: String
}

/// struct for passing parameters to the method [`get_log_digocean`]
#[derive(Clone, Debug, Default)]
pub struct GetLogDigoceanParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// The name for the real-time logging configuration.
    pub logging_digitalocean_name: String
}

/// struct for passing parameters to the method [`list_log_digocean`]
#[derive(Clone, Debug, Default)]
pub struct ListLogDigoceanParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32
}

/// struct for passing parameters to the method [`update_log_digocean`]
#[derive(Clone, Debug, Default)]
pub struct UpdateLogDigoceanParams {
    /// Alphanumeric string identifying the service.
    pub service_id: String,
    /// Integer identifying a service version.
    pub version_id: i32,
    /// The name for the real-time logging configuration.
    pub logging_digitalocean_name: String,
    /// The name for the real-time logging configuration.
    pub name: Option<String>,
    /// Where in the generated VCL the logging call should be placed. If not set, endpoints with `format_version` of 2 are placed in `vcl_log` and those with `format_version` of 1 are placed in `vcl_deliver`. 
    pub placement: Option<String>,
    /// The name of an existing condition in the configured endpoint, or leave blank to always execute.
    pub response_condition: Option<String>,
    /// A Fastly [log format string](https://docs.fastly.com/en/guides/custom-log-formats).
    pub format: Option<String>,
    /// The version of the custom logging format used for the configured endpoint. The logging call gets placed by default in `vcl_log` if `format_version` is set to `2` and in `vcl_deliver` if `format_version` is set to `1`. 
    pub format_version: Option<i32>,
    /// How the message should be formatted.
    pub message_type: Option<String>,
    /// A timestamp format
    pub timestamp_format: Option<String>,
    /// The codec used for compressing your logs. Valid values are `zstd`, `snappy`, and `gzip`. Specifying both `compression_codec` and `gzip_level` in the same API request will result in an error.
    pub compression_codec: Option<String>,
    /// How frequently log files are finalized so they can be available for reading (in seconds).
    pub period: Option<i32>,
    /// The level of gzip encoding when sending logs (default `0`, no compression). Specifying both `compression_codec` and `gzip_level` in the same API request will result in an error.
    pub gzip_level: Option<i32>,
    /// The name of the DigitalOcean Space.
    pub bucket_name: Option<String>,
    /// Your DigitalOcean Spaces account access key.
    pub access_key: Option<String>,
    /// Your DigitalOcean Spaces account secret key.
    pub secret_key: Option<String>,
    /// The domain of the DigitalOcean Spaces endpoint.
    pub domain: Option<String>,
    /// The path to upload logs to.
    pub path: Option<String>,
    /// A PGP public key that Fastly will use to encrypt your log files before writing them to disk.
    pub public_key: Option<String>
}


/// struct for typed errors of method [`create_log_digocean`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateLogDigoceanError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_log_digocean`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteLogDigoceanError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_log_digocean`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLogDigoceanError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`list_log_digocean`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ListLogDigoceanError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_log_digocean`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateLogDigoceanError {
    UnknownValue(serde_json::Value),
}


/// Create a DigitalOcean Space for a particular service and version.
pub async fn create_log_digocean(configuration: &mut configuration::Configuration, params: CreateLogDigoceanParams) -> Result<crate::models::LoggingDigitaloceanResponse, Error<CreateLogDigoceanError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;
    let name = params.name;
    let placement = params.placement;
    let response_condition = params.response_condition;
    let format = params.format;
    let format_version = params.format_version;
    let message_type = params.message_type;
    let timestamp_format = params.timestamp_format;
    let compression_codec = params.compression_codec;
    let period = params.period;
    let gzip_level = params.gzip_level;
    let bucket_name = params.bucket_name;
    let access_key = params.access_key;
    let secret_key = params.secret_key;
    let domain = params.domain;
    let path = params.path;
    let public_key = params.public_key;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/logging/digitalocean", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Fastly-Key", local_var_value);
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = name {
        local_var_form_params.insert("name", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = placement {
        local_var_form_params.insert("placement", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = response_condition {
        local_var_form_params.insert("response_condition", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = format {
        local_var_form_params.insert("format", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = format_version {
        local_var_form_params.insert("format_version", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = message_type {
        local_var_form_params.insert("message_type", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = timestamp_format {
        local_var_form_params.insert("timestamp_format", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = compression_codec {
        local_var_form_params.insert("compression_codec", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = period {
        local_var_form_params.insert("period", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = gzip_level {
        local_var_form_params.insert("gzip_level", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = bucket_name {
        local_var_form_params.insert("bucket_name", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = access_key {
        local_var_form_params.insert("access_key", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = secret_key {
        local_var_form_params.insert("secret_key", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = domain {
        local_var_form_params.insert("domain", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = path {
        local_var_form_params.insert("path", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = public_key {
        local_var_form_params.insert("public_key", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    if "POST" != "GET" && "POST" != "HEAD" {
      let headers = local_var_resp.headers();
      local_var_configuration.rate_limit_remaining = match headers.get("Fastly-RateLimit-Remaining") {
          Some(v) => v.to_str().unwrap().parse().unwrap(),
          None => configuration::DEFAULT_RATELIMIT,
      };
      local_var_configuration.rate_limit_reset = match headers.get("Fastly-RateLimit-Reset") {
          Some(v) => v.to_str().unwrap().parse().unwrap(),
          None => 0,
      };
    }

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateLogDigoceanError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Delete the DigitalOcean Space for a particular service and version.
pub async fn delete_log_digocean(configuration: &mut configuration::Configuration, params: DeleteLogDigoceanParams) -> Result<crate::models::InlineResponse200, Error<DeleteLogDigoceanError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;
    let logging_digitalocean_name = params.logging_digitalocean_name;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/logging/digitalocean/{logging_digitalocean_name}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id, logging_digitalocean_name=crate::apis::urlencode(logging_digitalocean_name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Fastly-Key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    if "DELETE" != "GET" && "DELETE" != "HEAD" {
      let headers = local_var_resp.headers();
      local_var_configuration.rate_limit_remaining = match headers.get("Fastly-RateLimit-Remaining") {
          Some(v) => v.to_str().unwrap().parse().unwrap(),
          None => configuration::DEFAULT_RATELIMIT,
      };
      local_var_configuration.rate_limit_reset = match headers.get("Fastly-RateLimit-Reset") {
          Some(v) => v.to_str().unwrap().parse().unwrap(),
          None => 0,
      };
    }

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<DeleteLogDigoceanError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Get the DigitalOcean Space for a particular service and version.
pub async fn get_log_digocean(configuration: &mut configuration::Configuration, params: GetLogDigoceanParams) -> Result<crate::models::LoggingDigitaloceanResponse, Error<GetLogDigoceanError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;
    let logging_digitalocean_name = params.logging_digitalocean_name;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/logging/digitalocean/{logging_digitalocean_name}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id, logging_digitalocean_name=crate::apis::urlencode(logging_digitalocean_name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Fastly-Key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    if "GET" != "GET" && "GET" != "HEAD" {
      let headers = local_var_resp.headers();
      local_var_configuration.rate_limit_remaining = match headers.get("Fastly-RateLimit-Remaining") {
          Some(v) => v.to_str().unwrap().parse().unwrap(),
          None => configuration::DEFAULT_RATELIMIT,
      };
      local_var_configuration.rate_limit_reset = match headers.get("Fastly-RateLimit-Reset") {
          Some(v) => v.to_str().unwrap().parse().unwrap(),
          None => 0,
      };
    }

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetLogDigoceanError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// List all of the DigitalOcean Spaces for a particular service and version.
pub async fn list_log_digocean(configuration: &mut configuration::Configuration, params: ListLogDigoceanParams) -> Result<Vec<crate::models::LoggingDigitaloceanResponse>, Error<ListLogDigoceanError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/logging/digitalocean", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Fastly-Key", local_var_value);
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    if "GET" != "GET" && "GET" != "HEAD" {
      let headers = local_var_resp.headers();
      local_var_configuration.rate_limit_remaining = match headers.get("Fastly-RateLimit-Remaining") {
          Some(v) => v.to_str().unwrap().parse().unwrap(),
          None => configuration::DEFAULT_RATELIMIT,
      };
      local_var_configuration.rate_limit_reset = match headers.get("Fastly-RateLimit-Reset") {
          Some(v) => v.to_str().unwrap().parse().unwrap(),
          None => 0,
      };
    }

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<ListLogDigoceanError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update the DigitalOcean Space for a particular service and version.
pub async fn update_log_digocean(configuration: &mut configuration::Configuration, params: UpdateLogDigoceanParams) -> Result<crate::models::LoggingDigitaloceanResponse, Error<UpdateLogDigoceanError>> {
    let local_var_configuration = configuration;

    // unbox the parameters
    let service_id = params.service_id;
    let version_id = params.version_id;
    let logging_digitalocean_name = params.logging_digitalocean_name;
    let name = params.name;
    let placement = params.placement;
    let response_condition = params.response_condition;
    let format = params.format;
    let format_version = params.format_version;
    let message_type = params.message_type;
    let timestamp_format = params.timestamp_format;
    let compression_codec = params.compression_codec;
    let period = params.period;
    let gzip_level = params.gzip_level;
    let bucket_name = params.bucket_name;
    let access_key = params.access_key;
    let secret_key = params.secret_key;
    let domain = params.domain;
    let path = params.path;
    let public_key = params.public_key;


    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/service/{service_id}/version/{version_id}/logging/digitalocean/{logging_digitalocean_name}", local_var_configuration.base_path, service_id=crate::apis::urlencode(service_id), version_id=version_id, logging_digitalocean_name=crate::apis::urlencode(logging_digitalocean_name));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_apikey) = local_var_configuration.api_key {
        let local_var_key = local_var_apikey.key.clone();
        let local_var_value = match local_var_apikey.prefix {
            Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
            None => local_var_key,
        };
        local_var_req_builder = local_var_req_builder.header("Fastly-Key", local_var_value);
    };
    let mut local_var_form_params = std::collections::HashMap::new();
    if let Some(local_var_param_value) = name {
        local_var_form_params.insert("name", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = placement {
        local_var_form_params.insert("placement", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = response_condition {
        local_var_form_params.insert("response_condition", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = format {
        local_var_form_params.insert("format", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = format_version {
        local_var_form_params.insert("format_version", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = message_type {
        local_var_form_params.insert("message_type", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = timestamp_format {
        local_var_form_params.insert("timestamp_format", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = compression_codec {
        local_var_form_params.insert("compression_codec", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = period {
        local_var_form_params.insert("period", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = gzip_level {
        local_var_form_params.insert("gzip_level", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = bucket_name {
        local_var_form_params.insert("bucket_name", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = access_key {
        local_var_form_params.insert("access_key", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = secret_key {
        local_var_form_params.insert("secret_key", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = domain {
        local_var_form_params.insert("domain", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = path {
        local_var_form_params.insert("path", local_var_param_value.to_string());
    }
    if let Some(local_var_param_value) = public_key {
        local_var_form_params.insert("public_key", local_var_param_value.to_string());
    }
    local_var_req_builder = local_var_req_builder.form(&local_var_form_params);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    if "PUT" != "GET" && "PUT" != "HEAD" {
      let headers = local_var_resp.headers();
      local_var_configuration.rate_limit_remaining = match headers.get("Fastly-RateLimit-Remaining") {
          Some(v) => v.to_str().unwrap().parse().unwrap(),
          None => configuration::DEFAULT_RATELIMIT,
      };
      local_var_configuration.rate_limit_reset = match headers.get("Fastly-RateLimit-Reset") {
          Some(v) => v.to_str().unwrap().parse().unwrap(),
          None => 0,
      };
    }

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateLogDigoceanError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

