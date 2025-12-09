//! Subscriptions
//! =============
//! The Subscriptions API allows you to create and manage recurring payments in your integration.

use super::PAYSTACK_BASE_URL;
use crate::{
    HttpClient, PaystackAPIError, PaystackResult, Response, SubscriptionRequest,
    SubscriptionResponseData,
};
use std::sync::Arc;

/// A struct to hold all the functions of the subscriptions API endpoint
#[derive(Debug, Clone)]
pub struct SubscriptionEndpoints<T: HttpClient + Default> {
    /// Paystack API key
    key: String,
    /// Base URL for the subscriptions route
    base_url: String,
    /// Http client for the route.
    http: Arc<T>,
}

impl<T: HttpClient + Default> SubscriptionEndpoints<T> {
    /// Creates a new SubscriptionEndpoints instance
    ///
    /// # Arguments
    /// * `key` - The Paystack API key
    /// * `http` - The HTTP client implementation to use for API requests
    ///
    /// # Returns
    /// A new SubscriptionEndpoints instance
    pub fn new(key: Arc<String>, http: Arc<T>) -> SubscriptionEndpoints<T> {
        let base_url = format!("{PAYSTACK_BASE_URL}/subscription");
        SubscriptionEndpoints {
            key: key.to_string(),
            base_url,
            http,
        }
    }

    /// Create a subscription on your integration
    pub async fn create_subscription(
        &self,
        subscription_request: SubscriptionRequest,
    ) -> PaystackResult<SubscriptionResponseData> {
        let url = self.base_url.to_string();
        let body = serde_json::to_value(subscription_request)
            .map_err(|e| PaystackAPIError::Subscription(e.to_string()))?;

        let response = self
            .http
            .post(&url, &self.key, &body)
            .await
            .map_err(|e| PaystackAPIError::Subaccount(e.to_string()))?;

        let parsed_response: Response<SubscriptionResponseData> =
            serde_json::from_str(&response)
                .map_err(|e| PaystackAPIError::Subscription(e.to_string()))?;

        Ok(parsed_response)
    }
}
