//! Error
//! ========
//! This file contains the structs and definitions of the errors in this crate.

/// Custom Error for the Paystack API
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum Error {
    /// Generic error, not used frequently
    #[error("Generic error: {0}")]
    Generic(String),
    /// Error associated with failed API request
    #[error("Failed Request Error: {0}")]
    FailedRequest(String),
    /// Error associated with Transaction operation
    #[error("Transaction Error: {0}")]
    Transaction(String),
    /// Error associated with Charge
    #[error("Charge Error: {0}")]
    Charge(String),
    /// Error associated with Transaction Split
    #[error("Transaction Split Error: {0}")]
    TransactionSplit(String),
    /// Error associated with Subaccount
    #[error("Subaccount Error: {0}")]
    Subaccount(String),
    /// Error for unsuccessful request to the Paystack API
    #[error("Request failed - Status Code: {0} Body: {1}")]
    RequestNotSuccessful(String, String),
    /// Error associated with failed parsing of response from the Paystack API
    #[error("Response parsing error: {0}")]
    ResponseParsing(String),
    /// Default HTTP error from the Reqwest crate
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
}
