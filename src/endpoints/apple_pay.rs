//! Apple Pay
//! THe Apple Pay API allows you register your application's top-level domain or subdomain.

use std::{marker::PhantomData, sync::Arc};

use serde_json::json;

use crate::{ApplePayResponseData, HttpClient, PaystackAPIError, PaystackResult, Response};

#[derive(Debug, Clone)]
pub struct ApplePayEndpoints<T: HttpClient + Default> {
    /// Paystack API key
    key: String,
    /// Base URL for the apple pay route
    base_url: String,
    /// Http client for the route
    http: Arc<T>,
}

impl<T: HttpClient + Default> ApplePayEndpoints<T> {
    /// Creates a new ApplePayEndpoints instance
    ///Creates a new ApplePayEndpoints instance
    ///
    /// # Arguments
    /// * `key` - The Paystack API key
    /// * `http` - The HTTP client implementation to use for API requests
    ///
    /// # Returns
    /// A new ApplePayEndpoints instance
    pub fn new(key: Arc<String>, http: Arc<T>) -> ApplePayEndpoints<T> {
        let base_url = String::from("https://api.paystack.co/apple-pay/domain");
        ApplePayEndpoints {
            key: key.to_string(),
            base_url,
            http,
        }
    }

    /// Register a top-level domain or subdomain for your Apple Pay integration.
    ///
    /// # Arguments
    /// * `domain_name` - The domain name to be registered with Apple Pay
    ///
    /// # Returns
    /// A Result containing the registration response or an error
    pub async fn register_domain(
        &self,
        domain_name: String,
    ) -> PaystackResult<PhantomData<String>> {
        let url = format!("{}", self.base_url);
        let body = json!({
            "domainName": domain_name
        });

        let response = self.http.post(&url, &self.key, &body).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<PhantomData<String>> =
                    serde_json::from_str(&response)
                        .map_err(|e| PaystackAPIError::ApplePay(e.to_string()))?;

                Ok(parsed_response)
            }
            Err(e) => Err(PaystackAPIError::ApplePay(e.to_string())),
        }
    }

    /// Lists all domains registered on your integration
    ///
    /// # Returns
    /// A Result containing the list of registered domains or an error
    pub async fn list_domains(&self) -> PaystackResult<ApplePayResponseData> {
        let url = format!("{}", self.base_url);

        let response = self.http.get(&url, &self.key, None).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<ApplePayResponseData> =
                    serde_json::from_str(&response)
                        .map_err(|e| PaystackAPIError::ApplePay(e.to_string()))?;

                Ok(parsed_response)
            }
            Err(e) => Err(PaystackAPIError::ApplePay(e.to_string())),
        }
    }

    pub async fn unregister_domain(
        &self,
        domain_name: String,
    ) -> PaystackResult<PhantomData<String>> {
        let url = format!("{}", self.base_url);
        let body = json!({
            "domainName": domain_name
        });

        let response = self.http.delete(&url, &self.key, &body).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<PhantomData<String>> =
                    serde_json::from_str(&response)
                        .map_err(|e| PaystackAPIError::ApplePay(e.to_string()))?;

                Ok(parsed_response)
            }
            Err(e) => Err(PaystackAPIError::ApplePay(e.to_string())),
        }
    }
}
