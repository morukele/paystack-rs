use async_trait::async_trait;
use serde_json::Value;
use std::fmt::{Debug, Display};

/// A predefined type for the query type in the HTTP client.
pub type Query<'a> = Vec<(&'a str, &'a str)>;

/// This trait is a collection of the stand HTTP methods for any client.
/// The aim of the trait is to abstract ways the HTTP implementation found in
/// different HTTP clients.
///
/// The goal is to give a level of flexibility to the user of the crate to work
/// with their preferred HTTP client.
/// To be as generic as possible, the U generic stands for the HTTP response.
/// Ideally, it should be bounded to specific traits common in all response.
/// TODO: Bound the U generic to the appropriate traits.

#[async_trait]
pub trait HttpClient: Debug + Default + Clone + Send {
    /// HTTP error
    type Error: Debug + Display;

    /// Send http get request
    async fn get(
        &self,
        url: &str,
        api_key: &str,
        query: Option<&Query>,
    ) -> Result<String, Self::Error>;
    /// Send http post request
    async fn post(&self, url: &str, api_key: &str, body: &Value) -> Result<String, Self::Error>;
    /// Send http put request
    async fn put(&self, url: &str, api_key: &str, body: &Value) -> Result<String, Self::Error>;
    /// Send http delete request
    async fn delete(&self, url: &str, api_key: &str, body: &Value) -> Result<String, Self::Error>;
}
