//! Client
//! =========
//! This file contains the Paystack API client, and it associated endpoints.
use crate::HttpClient;

/// This is the entry level struct for the paystack API.
/// it allows for authentication of the client
#[derive(Default, Debug, Clone)]
pub struct PaystackClient<T: HttpClient + Default> {
    /// Http client
    pub http: T,
    /// API keys
    pub key: String,
    /// Base Url
    pub base_url: String,
}

impl<T: HttpClient + Default> PaystackClient<T> {
    pub fn new(api_key: String, base_url: String) -> PaystackClient<T> {
        PaystackClient {
            key: api_key,
            base_url,
            http: T::default(),
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
        let url = "fake-url";
        let api_key = "fake-api-key";
        let paystack = PaystackClient::<ReqwestClient>::new(api_key.to_string(), url.to_string());

        // Assert
        assert_eq!(paystack.base_url, url);
        assert_eq!(paystack.key, api_key);
    }
}
