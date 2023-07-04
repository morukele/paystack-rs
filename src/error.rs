//! Error
//! ========
//! This file contaains the structs and definitions of the errors in this crate.

use std::fmt::Display;

use reqwest::StatusCode;

#[derive(thiserror::Error, Debug)]
pub enum PaystackError {
    #[error("Generic error: {0}")]
    Generic(String),

    #[error("Transaction Creation Error: {0}")]
    TransactionCreation(String),

    #[error("Request failed: `{0}`")]
    RequestNotSuccessful(#[from] RequestNotSuccessful),

    #[error("Response parsing error: {0}")]
    ResponseParsing(String),

    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
}

#[derive(Debug)]
/// Wrapper type which contains a failed request's status code and body
pub struct RequestNotSuccessful {
    /// Status code returned by the HTTP call to Paystack API
    pub status: StatusCode,
    /// Body returned by the HTTP call to the Paystack API
    pub body: String,
}

impl RequestNotSuccessful {
    /// Create a new unsucessful request error.
    pub fn new(status: StatusCode, body: String) -> Self {
        Self { status, body }
    }
}

impl std::error::Error for RequestNotSuccessful {}

impl Display for RequestNotSuccessful {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "StatusCode: {}, Body: {}", self.status, self.body)
    }
}
