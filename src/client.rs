//! Client
//! ===========
//! The file for the Paystack API client and it's associated functions
use crate::{Subaccount, Transaction, TransactionSplit};

/// This is the struct that allows you to authenticate to the PayStack API.
/// It contains the API key which allows you to interact with the API.
pub struct PaystackClient<'a> {
    /// Transaction API route
    pub transaction: Transaction<'a>,
    /// Transaction Split API route
    pub transaction_split: TransactionSplit<'a>,
    /// Subaccount API route
    pub subaccount: Subaccount<'a>,
}

impl<'a> PaystackClient<'a> {
    /// This method creates a new PayStack client with the specified API key.
    ///
    /// It takes the following parameters:
    ///     - key: Paystack API key.
    pub fn new(key: &'a str) -> PaystackClient<'a> {
        PaystackClient {
            transaction: Transaction::new(key),
            transaction_split: TransactionSplit::new(key),
            subaccount: Subaccount::new(key),
        }
    }
}
