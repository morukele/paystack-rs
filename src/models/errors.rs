//! Error
//! ========
//! This file contains the structs and definitions of the errors in this crate.
use thiserror::Error;

/// Custom Error for the Paystack API
#[derive(Error, Debug)]
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
    /// Error associated with terminal
    #[error("Terminal Error: {0}")]
    Terminal(String),
    /// Error associated with customer
    #[error("Customer Error: {0}")]
    Customer(String),
}

/// An error enum to hold errors from reqwest client
#[derive(Error, Debug)]
pub enum ReqwestError {
    /// Default HTTP error from the Reqwest crate
    #[error("request: {0}")]
    Reqwest(#[from] reqwest::Error),

    /// The initial request was successful, but the status code is in the 400
    /// and 500 range.
    /// This signifies that API cannot handle the request sent
    #[error("status code: {}", reqwest::Response::status(.0))]
    StatusCode(reqwest::Response),
}
