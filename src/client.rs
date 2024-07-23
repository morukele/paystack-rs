//! Client
//! =========
//! This file contains the Paystack API client, and it associated endpoints.
use crate::{HttpClient, TransactionEndpoints};
use std::env::Args;
use std::marker::PhantomData;
use std::sync::Arc;

/// This is the entry level struct for the paystack API.
/// it allows for authentication of the client
#[derive(Debug, Clone)]
pub struct PaystackClient<'a, T: HttpClient + Default> {
    /// Http client
    // use of Arc to ensure only one client is used, RC not used cause we want concurrency support.
    http: Arc<T>,
    /// API keys
    key: String,
    /// Transaction API route
    pub transaction: TransactionEndpoints<'a, T>,

    //Phantom data to keep compiler happy with lifetime
    phantom: PhantomData<&'a T>,
}

impl<'a, T: HttpClient + Default> PaystackClient<'a, T> {
    pub fn new(api_key: String) -> PaystackClient<'a, T> {
        let http = Arc::new(T::default());
        PaystackClient {
            key: api_key.clone(),
            http: Arc::clone(&http),
            transaction: TransactionEndpoints::new(api_key, Arc::clone(&http)),
            // use less data
            phantom: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::http::reqwest::ReqwestClient;

    #[test]
    fn create_a_new_paystack_instance() {
        // Set
        let api_key = "fake-api-key";
        let paystack = PaystackClient::<ReqwestClient>::new(api_key.to_string());

        // Assert
        assert_eq!(paystack.key, api_key);
    }
}
