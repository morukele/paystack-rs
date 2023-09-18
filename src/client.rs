//! Client
//! ===========
//! The file for the Paystack API client and it's associated functions
use crate::{Subaccount, Transaction, TransactionSplit};

/// This is the struct that allows you to authenticate to the PayStack API.
/// It contains the API key which allows you to interact with the API.
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
            transaction: Transaction::new(key.to_string()),
            transaction_split: TransactionSplit::new(key.to_string()),
            subaccount: Subaccount::new(key.to_string()),
        }
    }
}
