//! Client
//! =========
//! This file contains the Paystack API client, and it associated endpoints.
use crate::{HttpClient, TransactionEndpoints};
use std::marker::PhantomData;
use std::sync::Arc;

/// This is the entry level struct for the paystack API.
/// it allows for authentication of the client
pub struct PaystackClient<'a, T: HttpClient + Default> {
    /// Transaction API route
    pub transaction: TransactionEndpoints<'a, T>,

    //Phantom data to keep compiler happy with lifetime
    phantom: PhantomData<&'a T>,
}

impl<'a, T: HttpClient + Default> PaystackClient<'a, T> {
    pub fn new(api_key: String) -> PaystackClient<'a, T> {
        let http = Arc::new(T::default());
        PaystackClient {
            transaction: TransactionEndpoints::new(api_key, Arc::clone(&http)),
            // use less data
            phantom: PhantomData,
        }
    }
}
