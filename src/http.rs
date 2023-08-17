use std::collections::HashMap;
use std::env;
use dotenv::dotenv;
use reqwest::{Error, Response};
use serde_json::Value;

use crate::consts::{API_BASE, DEVICE};

pub(crate) async fn create_get_request(
    endpoint: &str,
    data: Option<HashMap<&str, &str>>,
    params: Option<Vec<(&str, &str)>>
) -> Result<Response, Error> {
    dotenv().expect(".env file not found");
    let url = API_BASE.to_owned() + &endpoint.replace("\"", "");
    let request_client = reqwest::Client::new();

    if !params.is_none() {
        reqwest::Url::parse_with_params(&url, params.unwrap())
            .unwrap()
            .as_str();
    }

    let token = env::var("TOKEN");
    let response = request_client
        .get(url)
        .json(&data)
        .header("X-Yandex-Music-Device", DEVICE)
        .header(reqwest::header::AUTHORIZATION, token.unwrap().as_str())
        .send()
        .await?;

    Ok(response)
}

pub(crate) async fn create_post_request(
    endpoint: &str,
    data: Option<HashMap<&str, &str>>,
    params: Option<Vec<(&str, &str)>>
) -> Result<Response, Error> {
    dotenv().expect(".env file not found");
    let url = API_BASE.to_owned() + &endpoint.replace("\"", "");

    if !params.is_none() {
        reqwest::Url::parse_with_params(&url, params.unwrap())
            .unwrap()
            .as_str();
    }

    let request_client = reqwest::Client::new();
    let token = env::var("TOKEN");
    let response = request_client
        .post(url)
        .json(&data)
        .header("X-Yandex-Music-Device", DEVICE)
        .header(reqwest::header::AUTHORIZATION, token.unwrap().as_str())
        .send()
        .await?;

    Ok(response)
}

fn parse_json(string_data: &str) -> Result<Value, Error> {
    let parsed_json: Value = serde_json::from_str(string_data).unwrap();

    Ok(parsed_json)
}
