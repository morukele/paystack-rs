//! Client
//! ===========
//! The file for the Paystack API client and it's associated functions

extern crate reqwest;
extern crate serde_json;

use crate::{Subaccount, Transaction, TransactionSplit};
use std::fmt::Debug;

/// This is the struct that allows you to authenticate to the PayStack API.
/// It contains the API key which allows you to interact with the API.
#[derive(Clone, Debug)]
pub struct PaystackClient {
    /// Transaction API route
    pub transaction: Transaction,
    /// Transaction Split API route
    pub transaction_split: TransactionSplit,
    /// Subaccount API route
    pub subaccount: Subaccount,
}

impl PaystackClient {
    /// This method creates a new PayStack client with the specified API key.
    ///
    /// It takes the following parameters:
    ///     - key: Paystack API key.
    pub fn new(key: String) -> Self {
        Self {
            transaction: Transaction {
                api_key: key.to_string(),
            },
            transaction_split: TransactionSplit {
                api_key: key.to_string(),
            },
            subaccount: Subaccount {
                api_key: key.to_string(),
            },
        }
    }
}
