//! Error
//! ========
//! This file contaains the structs and definitions of the errors in this crate.

use std::fmt::Display;

use reqwest::StatusCode;

/// Custom Error for the Paystack API
#[derive(thiserror::Error, Debug)]
pub enum PaystackError {
    /// Generic error, not used frequently
    #[error("Generic error: {0}")]
    Generic(String),

    /// Error associated with Transaction operation
    #[error("Transaction Error: {0}")]
    Transaction(String),

    /// Error associated with Charge
    #[error("Charge Error: {0}")]
    Charge(String),

    /// Error for unsuccessful request to the Paystack API
    #[error("Request failed: `{0}`")]
    RequestNotSuccessful(#[from] RequestNotSuccessful),

    /// Error associated with failed parsing of response from the Paystack API
    #[error("Response parsing error: {0}")]
    ResponseParsing(String),

    /// Default HTTP error from the Reqwest crate
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
