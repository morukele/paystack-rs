//! Client
//! =========
//! This file contains the Paystack API client, and it associated endpoints.
use crate::HttpClient;

/// This is the entry level struct for the paystack API.
/// it allows for authentication of the client
#[derive(Default, Debug, Clone, Copy)]
pub struct PaystackClient<'a, T: HttpClient + Default> {
    /// Http client
    pub http: T,
    /// API keys
    pub key: &'a str,
    /// Base Url
    pub base_url: &'a str,
}

impl<'a, T: HttpClient + Default> PaystackClient<'a, T> {
    pub fn new(api_key: &'a str, base_url: &'a str, http_client: T) -> PaystackClient<'a, T> {
        PaystackClient {
            key: api_key,
            base_url,
            http: http_client,
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
        let client = ReqwestClient::default();
        let paystack = PaystackClient::new(api_key, url, client);

        // Assert
        assert_eq!(paystack.base_url, url);
        assert_eq!(paystack.key, api_key);
    }
}
