/*
 * TheGamesDB API
 *
 * API Documentation
 *
 * The version of the OpenAPI document: 2.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method `games_by_game_id`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GamesByGameIdError {
    Status400(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `games_by_game_name`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GamesByGameNameError {
    Status400(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `games_by_game_name_v1`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GamesByGameNameV1Error {
    Status400(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `games_by_platform_id`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GamesByPlatformIdError {
    Status400(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `games_images`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GamesImagesError {
    Status400(),
    Status403(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method `games_updates`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GamesUpdatesError {
    Status400(),
    Status403(),
    UnknownValue(serde_json::Value),
}


/// can request additional information can be requestes through `fields` and `include` params
pub async fn games_by_game_id(configuration: &configuration::Configuration, apikey: &str, id: &str, fields: Option<&str>, include: Option<&str>, page: Option<i32>) -> Result<crate::models::GamesByGameId, Error<GamesByGameIdError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v1/Games/ByGameID", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("apikey", &apikey.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("id", &id.to_string())]);
    if let Some(ref local_var_str) = fields {
        local_var_req_builder = local_var_req_builder.query(&[("fields", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = include {
        local_var_req_builder = local_var_req_builder.query(&[("include", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GamesByGameIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// can request additional information can be requestes through `fields` and `include` params
pub async fn games_by_game_name(configuration: &configuration::Configuration, apikey: &str, name: &str, fields: Option<&str>, filter_platform: Option<&str>, include: Option<&str>, page: Option<i32>) -> Result<crate::models::GamesByGameId, Error<GamesByGameNameError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v1.1/Games/ByGameName", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("apikey", &apikey.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("name", &name.to_string())]);
    if let Some(ref local_var_str) = fields {
        local_var_req_builder = local_var_req_builder.query(&[("fields", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_platform {
        local_var_req_builder = local_var_req_builder.query(&[("filter[platform]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = include {
        local_var_req_builder = local_var_req_builder.query(&[("include", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GamesByGameNameError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// can request additional information can be requestes through `fields` and `include` params
pub async fn games_by_game_name_v1(configuration: &configuration::Configuration, apikey: &str, name: &str, fields: Option<&str>, filter_platform: Option<&str>, include: Option<&str>, page: Option<i32>) -> Result<crate::models::GamesByGameIdV1, Error<GamesByGameNameV1Error>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v1/Games/ByGameName", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("apikey", &apikey.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("name", &name.to_string())]);
    if let Some(ref local_var_str) = fields {
        local_var_req_builder = local_var_req_builder.query(&[("fields", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = filter_platform {
        local_var_req_builder = local_var_req_builder.query(&[("filter[platform]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = include {
        local_var_req_builder = local_var_req_builder.query(&[("include", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GamesByGameNameV1Error> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// can request additional information can be requested through `fields` and `include` params
pub async fn games_by_platform_id(configuration: &configuration::Configuration, apikey: &str, id: &str, fields: Option<&str>, include: Option<&str>, page: Option<i32>) -> Result<crate::models::GamesByGameId, Error<GamesByPlatformIdError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v1/Games/ByPlatformID", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("apikey", &apikey.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("id", &id.to_string())]);
    if let Some(ref local_var_str) = fields {
        local_var_req_builder = local_var_req_builder.query(&[("fields", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = include {
        local_var_req_builder = local_var_req_builder.query(&[("include", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GamesByPlatformIdError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// results can be filtered with `filter[type]` param
pub async fn games_images(configuration: &configuration::Configuration, apikey: &str, games_id: &str, filter_type: Option<&str>, page: Option<i32>) -> Result<crate::models::GamesImages, Error<GamesImagesError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v1/Games/Images", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("apikey", &apikey.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("games_id", &games_id.to_string())]);
    if let Some(ref local_var_str) = filter_type {
        local_var_req_builder = local_var_req_builder.query(&[("filter[type]", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GamesImagesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

pub async fn games_updates(configuration: &configuration::Configuration, apikey: &str, last_edit_id: i32, time: Option<i32>, page: Option<i32>) -> Result<crate::models::GamesUpdates, Error<GamesUpdatesError>> {

    let local_var_client = &configuration.client;

    let local_var_uri_str = format!("{}/v1/Games/Updates", configuration.base_path);
    let mut local_var_req_builder = local_var_client.get(local_var_uri_str.as_str());

    local_var_req_builder = local_var_req_builder.query(&[("apikey", &apikey.to_string())]);
    local_var_req_builder = local_var_req_builder.query(&[("last_edit_id", &last_edit_id.to_string())]);
    if let Some(ref local_var_str) = time {
        local_var_req_builder = local_var_req_builder.query(&[("time", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = page {
        local_var_req_builder = local_var_req_builder.query(&[("page", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_user_agent) = configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if local_var_status.is_success() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GamesUpdatesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

