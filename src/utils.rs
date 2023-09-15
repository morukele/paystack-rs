//! Utils
//! ============
//! This file contains utility sections that are used in different sections of the client

use std::fmt::Debug;
use reqwest::{Client, StatusCode, Response};
use serde::Serialize;
use crate::{PaystackResult, Error};

/// A function for sending GET request to a specified url
/// with optional query parameters using reqwest client.
pub async fn get_request(
    api_key: &String,
    url: &String,
    query: Option<Vec<(&str, String)>>,
) -> PaystackResult<Response> {
    let client = Client::new();
    let response = client
        .get(url)
        .query(&query)
        .bearer_auth(api_key)
        .header("Content-Type", "application/json")
        .send()
        .await;

    match response {
        Ok(response) => match response.status() {
            StatusCode::OK => Ok(response),
            _ => Err(Error::RequestNotSuccessful(response.status().to_string(), response.text().await?))
        },
        Err(err) => Err(Error::Generic(err.to_string())),
    }
}

/// A function for sending POST requests to a specified url
/// using the reqwest client.
pub async fn post_request<T>(api_key: &String, url: &String, body: T) -> PaystackResult<Response>
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
        Ok(response) => match response.status() {
            StatusCode::OK => Ok(response),
            _ => {
                Err(Error::RequestNotSuccessful(response.status().to_string(), response.text().await?))
            }
        },
        Err(err) => Err(Error::Generic(err.to_string())),
    }
}

/// A function for sending PUT requests to a specified url
/// using the reqwest client.
pub async fn put_request<T>(api_key: &String, url: &String, body: T) -> PaystackResult<Response>
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
        Ok(response) => match response.status() {
            StatusCode::OK => Ok(response),
            _ => {
                Err(Error::RequestNotSuccessful(response.status().to_string(), response.text().await?))
            }
        },
        Err(err) => Err(Error::Generic(err.to_string())),
    }
}

/// A function for sending DELETE requests to a specified url
/// using the reqwest client.
pub async fn delete_request<T>(api_key: &String, url: &String, body: T) -> PaystackResult<Response>
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
        Ok(response) => match response.status() {
            StatusCode::OK => Ok(response),
            _ => {
                Err(Error::RequestNotSuccessful(response.status().to_string(), response.text().await?))
            }
        },
        Err(err) => Err(Error::Generic(err.to_string())),
    }
}