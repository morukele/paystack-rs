//! Client
//! =========
//! This file contains the Paystack API client, and it associated endpoints.
use crate::HttpClient;

/// This is the entry level struct for the paystack API.
/// it allows for authentication of the client
pub struct PaystackClient<'a, Http: HttpClient> {
    /// Http client
    pub client: Http,
    /// API keys
    pub key: &'a str,
}

impl<'a, Http: HttpClient> PaystackClient<'a, Http> {}
