//! Subaccounts
//! ===========
//! The Subaccounts API allows you to create and manage subaccounts on your integration.
//! Subaccounts can be used to split payment between two accounts (your main account and a subaccount).

use crate::{
    HttpClient, PaystackAPIError, PaystackResult, Response, SubaccountRequest,
    SubaccountsResponseData,
};
use std::sync::Arc;

/// A struct to hold all functions in the subaccount API route
#[derive(Debug, Clone)]
pub struct SubaccountEndpoints<T: HttpClient + Default> {
    /// Paystack API Key
    key: String,
    /// Base URL for the transaction route
    base_url: String,
    /// Http client for the route
    http: Arc<T>,
}

impl<T: HttpClient + Default> SubaccountEndpoints<T> {
    /// Creates a new SubaccountEndpoints instance
    ///
    /// # Arguments
    /// * `key` - The Paystack API key
    /// * `http` - The HTTP client implementation to use for API requests
    ///
    /// # Returns
    /// A new SubaccountEndpoints instance
    pub fn new(key: Arc<String>, http: Arc<T>) -> SubaccountEndpoints<T> {
        let base_url = String::from("https://api.paystack.co/subaccount");
        SubaccountEndpoints {
            key: key.to_string(),
            base_url,
            http,
        }
    }

    /// Create a subaccount on your integration
    ///
    /// # Arguments
    /// * `subaccount_request` - The request data to create the subaccount.
    ///   It should be created with the `SubaccountRequestBuilder` struct.
    ///
    /// # Returns
    /// A Result containing the subaccount response data or an error
    pub async fn create_subaccount(
        &self,
        subaccount_request: SubaccountRequest,
    ) -> PaystackResult<SubaccountsResponseData> {
        let url = self.base_url.to_string();
        let body = serde_json::to_value(subaccount_request)
            .map_err(|e| PaystackAPIError::Subaccount(e.to_string()))?;

        let response = self
            .http
            .post(&url, &self.key, &body)
            .await
            .map_err(|e| PaystackAPIError::Subaccount(e.to_string()))?;

        let parsed_response: Response<SubaccountsResponseData> = serde_json::from_str(&response)
            .map_err(|e| PaystackAPIError::Subaccount(e.to_string()))?;
        Ok(parsed_response)
    }
}
