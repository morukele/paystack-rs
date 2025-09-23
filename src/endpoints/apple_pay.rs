//! Apple Pay
//! THe Apple Pay API allows you register your application's top-level domain or subdomain.

use super::PAYSTACK_BASE_URL;
use crate::{ApplePayResponseData, HttpClient, PaystackAPIError, PaystackResult};
use serde_json::json;
use std::{marker::PhantomData, sync::Arc};

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
        let base_url = format!("{PAYSTACK_BASE_URL}/apple-pay/domain");
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
        let url = &self.base_url;
        let body = json!({
            "domainName": domain_name
        });

        let response = self
            .http
            .post(url, &self.key, &body)
            .await
            .map_err(|e| PaystackAPIError::ApplePay(e.to_string()))?;

        let parsed_response = serde_json::from_str(&response)
            .map_err(|e| PaystackAPIError::ApplePay(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Lists all domains registered on your integration
    ///
    /// # Returns
    /// A Result containing the list of registered domains or an error
    pub async fn list_domains(&self) -> PaystackResult<ApplePayResponseData> {
        let url = &self.base_url;

        let response = self
            .http
            .get(url, &self.key, None)
            .await
            .map_err(|e| PaystackAPIError::ApplePay(e.to_string()))?;

        let parsed_response = serde_json::from_str(&response)
            .map_err(|e| PaystackAPIError::ApplePay(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Unregister a top-level domain or subdomain previously used for your Apple Pay integration.
    ///
    /// # Arguments
    /// * `domain_name` - The name of the domain to unregister
    ///
    /// # Returns
    /// A result containing a success message without data.
    pub async fn unregister_domain(
        &self,
        domain_name: String,
    ) -> PaystackResult<PhantomData<String>> {
        let url = &self.base_url;
        let body = json!({
            "domainName": domain_name
        });

        let response = self
            .http
            .delete(url, &self.key, &body)
            .await
            .map_err(|e| PaystackAPIError::ApplePay(e.to_string()))?;

        let parsed_response = serde_json::from_str(&response)
            .map_err(|e| PaystackAPIError::ApplePay(e.to_string()))?;

        Ok(parsed_response)
    }
}
