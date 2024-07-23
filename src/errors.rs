//! Error
//! ========
//! This file contains the structs and definitions of the errors in this crate.
use thiserror::Error;

/// Custom Error for the Paystack API
#[derive(Error, Debug)]
#[non_exhaustive]
pub enum PaystackAPIError {
    /// Generic error, not used frequently
    #[error("Generic error: {0}")]
    Generic(String),
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
