//! Client
//! ===========
//! The file for the Paystack API client and it's associated functions
use crate::{
    CustomerEndpoints, SubaccountEndpoints, TerminalEndpoints, TransactionEndpoints,
    TransactionSplitEndpoints,
};

/// This is the struct that allows you to authenticate to the PayStack API.
/// It contains the API key which allows you to interact with the API.
pub struct PaystackClient<'a> {
    /// Transaction API route
    pub transaction: TransactionEndpoints<'a>,
    /// Transaction Split API route
    pub transaction_split: TransactionSplitEndpoints<'a>,
    /// Subaccount API route
    pub subaccount: SubaccountEndpoints<'a>,
    /// Terminal API route
    pub terminal: TerminalEndpoints<'a>,
    /// Customer API route
    pub customer: CustomerEndpoints<'a>,
}

impl<'a> PaystackClient<'a> {
    /// This method creates a new PayStack client with the specified API key.
    ///
    /// It takes the following parameters:
    ///     - key: Paystack API key.
    pub fn new(key: &'a str) -> PaystackClient<'a> {
        PaystackClient {
            transaction: TransactionEndpoints::new(key),
            transaction_split: TransactionSplitEndpoints::new(key),
            subaccount: SubaccountEndpoints::new(key),
            terminal: TerminalEndpoints::new(key),
            customer: CustomerEndpoints::new(key),
        }
    }
}
