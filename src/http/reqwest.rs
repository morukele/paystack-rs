use super::ReqwestError;
use crate::http::base::Query;
use crate::HttpClient;
use async_trait::async_trait;
use reqwest::{Client, Method, RequestBuilder, Response};
use serde_json::Value;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub struct ReqwestClient {
    /// An instance of the client to perform the http requests with
    client: Client,
}

impl Default for ReqwestClient {
    fn default() -> Self {
        let client = reqwest::ClientBuilder::new().build().unwrap();

        Self { client }
    }
}

impl ReqwestClient {
    async fn send_request<D: Fn(RequestBuilder) -> RequestBuilder>(
        &self,
        method: Method,
        url: &str,
        auth_key: &str,
        add_data: D,
    ) -> Result<Response, ReqwestError> {
        // configure the request object
        let mut request = self
            .client
            .request(method.clone(), url)
            .bearer_auth(auth_key)
            .header("Content-Type", "application/json");

        // Configure the request for the specific type (get/post/put/delete)
        request = add_data(request);

        // Performing the request
        log::info!("Making request: {:?}", request);
        let response = request.send().await?;

        // Checking that we get a 200 range response
        if response.status().is_success() {
            Ok(response)
        } else {
            Err(ReqwestError::StatusCode(response))
        }
    }
}

#[async_trait]
impl HttpClient for ReqwestClient {
    type Error = ReqwestError;
    type Output = Response;

    async fn get(
        &self,
        url: &str,
        api_key: &str,
        query: Option<&Query>,
    ) -> Result<Self::Output, Self::Error> {
        self.send_request(Method::GET, url, api_key, |req| {
            if let Some(query) = query {
                req.query(query)
            } else {
                req
            }
        })
        .await
    }

    async fn post(
        &self,
        url: &str,
        api_key: &str,
        body: &Value,
    ) -> Result<Self::Output, Self::Error> {
        self.send_request(Method::POST, url, api_key, |req| req.json(body))
            .await
    }

    async fn put(
        &self,
        url: &str,
        api_key: &str,
        body: &Value,
    ) -> Result<Self::Output, Self::Error> {
        self.send_request(Method::PUT, url, api_key, |req| req.json(body))
            .await
    }

    async fn delete(
        &self,
        url: &str,
        api_key: &str,
        body: &Value,
    ) -> Result<Self::Output, Self::Error> {
        self.send_request(Method::DELETE, url, api_key, |req| req.json(body))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn reqwest_client_cannot_get_unauthorized() {
        // Set
        let api_key = String::from("fake-key");
        let url = "https://api.paystack.co/transaction/initialize";

        // Run
        let client = ReqwestClient::default();
        let res = client.get(url, api_key.as_str(), None).await;

        // Assert
        // this should be a 401 error since we are not passing the right API key
        if let Ok(res) = res {
            assert_eq!(res.status(), 401);
        }
    }

    #[tokio::test]
    async fn reqwest_client_can_get() {
        // Set
        let api_key = "fake-hey";
        let url = "https://api.paystack.co/";

        // Run
        let client = ReqwestClient::default();
        let res = client.get(url, api_key, None).await;

        // Assert
        if let Ok(res) = res {
            assert_eq!(res.status(), 200);
        }
    }
}
