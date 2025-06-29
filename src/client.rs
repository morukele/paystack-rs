//! Client
//! =========
//! This file contains the Paystack API client, and it associated endpoints.
use crate::{
    CustomersEndpoints, DedicatedVirtualAccountEndpoints, HttpClient, SubaccountEndpoints,
    TerminalEndpoints, TransactionEndpoints, TransactionSplitEndpoints, VirtualTerminalEndpoints,
};
use std::sync::Arc;

/// This is the entry level struct for the paystack API.
/// it allows for authentication of the client
pub struct PaystackClient<T: HttpClient + Default> {
    /// Transaction API route
    pub transactions: TransactionEndpoints<T>,
    /// Transaction Split API route
    pub transaction_split: TransactionSplitEndpoints<T>,
    /// Subaccount API route
    pub subaccount: SubaccountEndpoints<T>,
    /// Terminal API route
    pub terminal: TerminalEndpoints<T>,
    /// Virutal Terminal API route
    pub virutal_terminal: VirtualTerminalEndpoints<T>,
    /// Customers API route
    pub customers: CustomersEndpoints<T>,
    /// Dedicated Virtual Account API route
    pub dedicated_virtual_account: DedicatedVirtualAccountEndpoints<T>,
}

impl<T: HttpClient + Default> PaystackClient<T> {
    pub fn new(api_key: String) -> PaystackClient<T> {
        let http = Arc::new(T::default());
        let key = Arc::new(api_key);
        PaystackClient {
            transactions: TransactionEndpoints::new(Arc::clone(&key), Arc::clone(&http)),
            transaction_split: TransactionSplitEndpoints::new(Arc::clone(&key), Arc::clone(&http)),
            subaccount: SubaccountEndpoints::new(Arc::clone(&key), Arc::clone(&http)),
            terminal: TerminalEndpoints::new(Arc::clone(&key), Arc::clone(&http)),
            virutal_terminal: VirtualTerminalEndpoints::new(Arc::clone(&key), Arc::clone(&http)),
            customers: CustomersEndpoints::new(Arc::clone(&key), Arc::clone(&http)),
            dedicated_virtual_account: DedicatedVirtualAccountEndpoints::new(
                Arc::clone(&key),
                Arc::clone(&http),
            ),
        }
    }
}
