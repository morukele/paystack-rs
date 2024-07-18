use reqwest::Body;
use serde::Deserialize;
use std::fmt;
use std::fmt::Debug;
use std::future::Future;

pub type Query<'a> = Vec<(&'a str, &'a str)>;

/// This trait is a collection of the stand HTTP methods for any client.
/// The aim of the trait is to abstract ways the HTTP implementation found in
/// different HTTP clients.
///
/// The goal is to give a level of flexibility to the user of the crate to work
/// with their preferred HTTP client.
pub trait HttpClient: Send + Default + Clone + fmt::Debug {
    /// HTTP error
    type Error;
    /// Send http get request
    fn get(
        &self,
        url: &str,
        api_key: &str,
        query: Option<Query>,
    ) -> impl Future<Output = Result<String, Self::Error>> + Send;
    /// Send http post request
    fn post<'a, T: Deserialize<'a> + Debug>(
        &self,
        url: &str,
        api_key: &str,
        body: T,
    ) -> impl Future<Output = Result<String, Self::Error>> + Send;
    /// Send http put request
    fn put<'a, T: Deserialize<'a> + Debug>(
        &self,
        url: &str,
        api_key: &str,
        body: T,
    ) -> impl Future<Output = Result<String, Self::Error>> + Send;
    /// Send http delete request
    fn delete<'a, T: Debug + Deserialize<'a>>(
        &self,
        url: &str,
        api_key: &str,
        body: T,
    ) -> impl Future<Output = Result<String, Self::Error>> + Send;
}
