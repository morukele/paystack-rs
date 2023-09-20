//! Utils
//! ============
//! This file contains utility sections that are used in different sections of the client

use reqwest::{Client, Error, Response};
use serde::Serialize;
use std::fmt::Debug;

/// A function for sending GET request to a specified url
/// with optional query parameters using reqwest client.
pub async fn get_request(
    api_key: &str,
    url: &str,
    query: Option<Vec<(&str, &str)>>,
) -> Result<Response, Error> {
    let client = Client::new();
    let response = client
        .get(url)
        .query(&query)
        .bearer_auth(api_key)
        .header("Content-Type", "application/json")
        .send()
        .await;

    match response {
        Ok(response) => Ok(response),
        Err(err) => Err(err),
    }
}

/// A function for sending POST requests to a specified url
/// using the reqwest client.
pub async fn post_request<T>(api_key: &str, url: &str, body: T) -> Result<Response, Error>
where
    T: Debug + Serialize,
{
    let client = Client::new();
    let response = client
        .post(url)
        .bearer_auth(api_key)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await;

    match response {
        Ok(response) => Ok(response),
        Err(err) => Err(err),
    }
}

/// A function for sending PUT requests to a specified url
/// using the reqwest client.
pub async fn put_request<T>(api_key: &str, url: &str, body: T) -> Result<Response, Error>
where
    T: Debug + Serialize,
{
    let client = Client::new();
    let response = client
        .put(url)
        .bearer_auth(api_key)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await;

    match response {
        Ok(response) => Ok(response),
        Err(err) => Err(err),
    }
}

/// A function for sending DELETE requests to a specified url
/// using the reqwest client.
pub async fn delete_request<T>(api_key: &str, url: &str, body: T) -> Result<Response, Error>
where
    T: Debug + Serialize,
{
    let client = Client::new();
    let response = client
        .delete(url)
        .bearer_auth(api_key)
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await;

    match response {
        Ok(response) => Ok(response),
        Err(err) => Err(err),
    }
}
