//! Client
//! =========
//! This file contains the Paystack API client, and it associated endpoints.
use crate::{
    ApplePayEndpoints, CustomersEndpoints, DedicatedVirtualAccountEndpoints, HttpClient,
    PlansEndpoints, SubaccountEndpoints, SubscriptionEndpoints, TerminalEndpoints,
    TransactionEndpoints, TransactionSplitEndpoints, VirtualTerminalEndpoints,
};
use std::sync::Arc;

/// This is the entry level struct for the paystack API.
/// it allows for authentication of the client
pub struct PaystackClient<T: HttpClient + Default> {
    /// Transaction API route
    pub transactions: TransactionEndpoints<T>,
    /// Transaction Split API route
    pub transaction_splits: TransactionSplitEndpoints<T>,
    /// Subaccount API route
    pub subaccounts: SubaccountEndpoints<T>,
    /// Terminal API route
    pub terminal: TerminalEndpoints<T>,
    /// Virutal Terminal API route
    pub virutal_terminal: VirtualTerminalEndpoints<T>,
    /// Customers API route
    pub customers: CustomersEndpoints<T>,
    /// Dedicated Virtual Account API route
    pub dedicated_virtual_accounts: DedicatedVirtualAccountEndpoints<T>,
    /// Apple Pay API route
    pub apple_pay: ApplePayEndpoints<T>,
    /// Plans API route
    pub plans: PlansEndpoints<T>,
    /// Subscription API route
    pub subscriptions: SubscriptionEndpoints<T>,
}

impl<T: HttpClient + Default> PaystackClient<T> {
    pub fn new(api_key: String) -> PaystackClient<T> {
        let http = Arc::new(T::default());
        let key = Arc::new(api_key);
        PaystackClient {
            transactions: TransactionEndpoints::new(Arc::clone(&key), Arc::clone(&http)),
            transaction_splits: TransactionSplitEndpoints::new(Arc::clone(&key), Arc::clone(&http)),
            subaccounts: SubaccountEndpoints::new(Arc::clone(&key), Arc::clone(&http)),
            terminal: TerminalEndpoints::new(Arc::clone(&key), Arc::clone(&http)),
            virutal_terminal: VirtualTerminalEndpoints::new(Arc::clone(&key), Arc::clone(&http)),
            customers: CustomersEndpoints::new(Arc::clone(&key), Arc::clone(&http)),
            dedicated_virtual_accounts: DedicatedVirtualAccountEndpoints::new(
                Arc::clone(&key),
                Arc::clone(&http),
            ),
            apple_pay: ApplePayEndpoints::new(Arc::clone(&key), Arc::clone(&http)),
            plans: PlansEndpoints::new(Arc::clone(&key), Arc::clone(&http)),
            subscriptions: SubscriptionEndpoints::new(Arc::clone(&key), Arc::clone(&http)),
        }
    }
}
