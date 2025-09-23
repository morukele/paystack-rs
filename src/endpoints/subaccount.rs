//! Subaccounts
//! ===========
//! The Subaccounts API allows you to create and manage subaccounts on your integration.
//! Subaccounts can be used to split payment between two accounts (your main account and a subaccount).

use super::PAYSTACK_BASE_URL;
use crate::{
    CreateSubaccountRequest, HttpClient, PaystackAPIError, PaystackResult, Response,
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
        let base_url = format!("{PAYSTACK_BASE_URL}/subaccount");
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
    ///   It should be created with the `CreateSubaccountRequestBuilder` struct.
    ///
    /// # Returns
    /// A Result containing the subaccount response data or an error
    pub async fn create_subaccount(
        &self,
        subaccount_request: CreateSubaccountRequest,
    ) -> PaystackResult<SubaccountsResponseData> {
        let url = &self.base_url;
        let body = serde_json::to_value(subaccount_request)
            .map_err(|e| PaystackAPIError::Subaccount(e.to_string()))?;

        let response = self
            .http
            .post(url, &self.key, &body)
            .await
            .map_err(|e| PaystackAPIError::Subaccount(e.to_string()))?;

        let parsed_response: Response<SubaccountsResponseData> = serde_json::from_str(&response)
            .map_err(|e| PaystackAPIError::Subaccount(e.to_string()))?;
        Ok(parsed_response)
    }

    /// List subaccounts available on your integration.
    ///
    /// # Arguments
    /// * `per_page` - Optional number of subaccounts to return per page. Defaults to 50 if None.
    /// * `page` - Specify exactly what page you want to retrieve. Defaults to 1 if None.
    ///
    /// # Returns
    /// A Result containing a vector of subaccount data or an error.
    pub async fn list_subaccounts(
        &self,
        per_page: Option<u32>,
        page: Option<u32>,
    ) -> PaystackResult<Vec<SubaccountsResponseData>> {
        let url = self.base_url.to_string();

        let per_page = per_page.unwrap_or(50).to_string();
        let page = page.unwrap_or(1).to_string();
        let query = vec![("perPage", per_page.as_str()), ("page", page.as_str())];

        let response = self
            .http
            .get(&url, &self.key, Some(&query))
            .await
            .map_err(|e| PaystackAPIError::Subaccount(e.to_string()))?;

        let parsed_response: Response<Vec<SubaccountsResponseData>> =
            serde_json::from_str(&response)
                .map_err(|e| PaystackAPIError::Subaccount(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Get the details of a subaccount on your integration
    ///
    /// # Arguments
    /// * `id_or_code` - The subaccount ID or code you want to fetch
    ///
    /// # Returns
    /// A Result containing the details of the subaccount or an error.
    pub async fn fetch_subaccount(
        &self,
        id_or_code: String,
    ) -> PaystackResult<SubaccountsResponseData> {
        let url = format!("{}/{}", self.base_url, id_or_code);

        let response = self
            .http
            .get(&url, &self.key, None)
            .await
            .map_err(|e| PaystackAPIError::Subaccount(e.to_string()))?;

        let parsed_response: Response<SubaccountsResponseData> = serde_json::from_str(&response)
            .map_err(|e| PaystackAPIError::Subaccount(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Update a subaccount details in your integration
    ///
    /// # Arguments
    /// * `id_or_code` - Subaccount's ID or code
    /// * `update_request` - The request data to update the subaccount.
    ///   It should be created with the `CreateSubaccountRequestBuilder` struct.
    ///
    /// # Returns
    /// A Result containing the updated subaccount response data or an error
    pub async fn update_subaccount(
        &self,
        id_or_code: String,
        update_request: CreateSubaccountRequest,
    ) -> PaystackResult<SubaccountsResponseData> {
        let url = format!("{}/{}", self.base_url, id_or_code);
        let body = serde_json::to_value(update_request)
            .map_err(|e| PaystackAPIError::Subaccount(e.to_string()))?;

        let response = self
            .http
            .put(&url, &self.key, &body)
            .await
            .map_err(|e| PaystackAPIError::Subaccount(e.to_string()))?;

        let parsed_response: Response<SubaccountsResponseData> = serde_json::from_str(&response)
            .map_err(|e| PaystackAPIError::Subaccount(e.to_string()))?;

        Ok(parsed_response)
    }
}
