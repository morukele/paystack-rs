//! Transaction Split
//! =================
//! The Transaction Splits API enables merchants split the settlement for a
//! transaction across their payout account, and one or more subaccounts.

use super::PAYSTACK_BASE_URL;
use crate::{
    DeleteSubAccountBody, HttpClient, PaystackAPIError, PaystackResult, Response, SubaccountBody,
    TransactionSplitRequest, TransactionSplitResponseData, UpdateTransactionSplitRequest,
};
use std::sync::Arc;

/// A struct to hold all the functions of the transaction split API endpoint
#[derive(Debug, Clone)]
pub struct TransactionSplitEndpoints<T: HttpClient + Default> {
    /// Paystack API Key
    key: String,
    /// Base URL for the transaction route
    base_url: String,
    /// Http client for the route
    http: Arc<T>,
}

impl<T: HttpClient + Default> TransactionSplitEndpoints<T> {
    /// Creates a new TransactionSplitEndpoints instance
    ///
    /// # Arguments
    /// * `key` - The Paystack API key
    /// * `http` - The HTTP client implementation to use for API requests
    ///
    /// # Returns
    /// A new TransactionSplitEndpoints instance
    pub fn new(key: Arc<String>, http: Arc<T>) -> TransactionSplitEndpoints<T> {
        let base_url = format!("{PAYSTACK_BASE_URL}/split");
        TransactionSplitEndpoints {
            key: key.to_string(),
            base_url,
            http,
        }
    }

    /// Creates a split payment on your integration
    ///
    /// # Arguments
    /// * `split_body` - The request data to create the split payment.
    ///   It should be created with a `TransactionSplitRequest` struct.
    ///
    /// # Returns
    /// A Result containing the transaction split response data or an error
    pub async fn create_transaction_split(
        &self,
        split_body: TransactionSplitRequest,
    ) -> PaystackResult<TransactionSplitResponseData> {
        let url = &self.base_url;
        let body = serde_json::to_value(split_body)
            .map_err(|e| PaystackAPIError::TransactionSplit(e.to_string()))?;

        let response = self
            .http
            .post(url, &self.key, &body)
            .await
            .map_err(|e| PaystackAPIError::TransactionSplit(e.to_string()))?;

        let parsed_response: Response<TransactionSplitResponseData> =
            serde_json::from_str(&response)
                .map_err(|e| PaystackAPIError::TransactionSplit(e.to_string()))?;
        Ok(parsed_response)
    }

    /// Lists transaction splits available on your integration
    ///
    /// # Arguments
    /// * `split_name` - Optional name of the split to retrieve
    /// * `split_active` - Optional status of the split to retrieve
    ///
    /// # Returns
    /// A Result containing a vector of transaction split response data or an error
    pub async fn list_transaction_splits(
        &self,
        split_name: Option<&str>,
        split_active: Option<bool>,
    ) -> PaystackResult<Vec<TransactionSplitResponseData>> {
        let url = &self.base_url;

        // Specify a default option for active splits
        let split_active = match split_active {
            Some(active) => active.to_string(),
            None => "".to_string(),
        };

        let query = vec![
            ("name", split_name.unwrap_or("")),
            ("active", &split_active),
        ];

        let response = self
            .http
            .get(url, &self.key, Some(&query))
            .await
            .map_err(|e| PaystackAPIError::TransactionSplit(e.to_string()))?;

        let parsed_response: Response<Vec<TransactionSplitResponseData>> =
            serde_json::from_str(&response)
                .map_err(|e| PaystackAPIError::TransactionSplit(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Gets details of a split on your integration
    ///
    /// # Arguments
    /// * `split_id` - ID of the transaction split to fetch
    ///
    /// # Returns
    /// A Result containing the transaction split response data or an error
    pub async fn fetch_transaction_split(
        &self,
        split_id: &str,
    ) -> PaystackResult<TransactionSplitResponseData> {
        let url = format!("{}/{}", self.base_url, split_id);

        let response = self
            .http
            .get(&url, &self.key, None)
            .await
            .map_err(|e| PaystackAPIError::TransactionSplit(e.to_string()))?;

        let parsed_response: Response<TransactionSplitResponseData> =
            serde_json::from_str(&response)
                .map_err(|e| PaystackAPIError::TransactionSplit(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Updates a transaction split's details on your integration
    ///
    /// # Arguments
    /// * `split_id` - ID of the split to update
    /// * `update_body` - The update data for the transaction split.
    ///   It should be created with the `UpdateTransactionSplitRequestBuilder` struct.
    ///
    /// # Returns
    /// A Result containing the updated transaction split response data or an error
    pub async fn update_transaction_split(
        &self,
        split_id: &str,
        update_body: UpdateTransactionSplitRequest,
    ) -> PaystackResult<TransactionSplitResponseData> {
        let url = format!("{}/{}", self.base_url, split_id);
        let body = serde_json::to_value(update_body)
            .map_err(|e| PaystackAPIError::TransactionSplit(e.to_string()))?;

        let response = self
            .http
            .put(&url, &self.key, &body)
            .await
            .map_err(|e| PaystackAPIError::TransactionSplit(e.to_string()))?;

        let parsed_response: Response<TransactionSplitResponseData> =
            serde_json::from_str(&response)
                .map_err(|e| PaystackAPIError::TransactionSplit(e.to_string()))?;
        Ok(parsed_response)
    }

    /// Adds a subaccount to a transaction split or updates an existing subaccount's share
    ///
    /// # Arguments
    /// * `split_id` - ID of the transaction split to update
    /// * `body` - The subaccount data to add or update.
    ///   It should be created with a `SubaccountBody` struct.
    ///
    /// # Returns
    /// A Result containing the transaction split response data or an error
    pub async fn add_or_update_subaccount_split(
        &self,
        split_id: &str,
        body: SubaccountBody,
    ) -> PaystackResult<TransactionSplitResponseData> {
        let url = format!("{}/{}/subaccount/add", self.base_url, split_id);
        let body = serde_json::to_value(body)
            .map_err(|e| PaystackAPIError::TransactionSplit(e.to_string()))?;

        let response = self
            .http
            .post(&url, &self.key, &body)
            .await
            .map_err(|e| PaystackAPIError::TransactionSplit(e.to_string()))?;

        let parsed_response: Response<TransactionSplitResponseData> =
            serde_json::from_str(&response)
                .map_err(|e| PaystackAPIError::TransactionSplit(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Removes a subaccount from a transaction split
    ///
    /// # Arguments
    /// * `split_id` - ID of the transaction split.
    /// * `subaccount` - The subaccount data to remove.
    ///   It should be created with a `DeleteSubAccountBody` struct.
    ///
    /// # Returns
    /// A Result containing a success message or an error
    pub async fn remove_subaccount_from_transaction_split(
        &self,
        split_id: &str,
        subaccount: DeleteSubAccountBody,
    ) -> PaystackResult<String> {
        let url = format!("{}/{}/subaccount/remove", self.base_url, split_id);
        let body = serde_json::to_value(subaccount)
            .map_err(|e| PaystackAPIError::TransactionSplit(e.to_string()))?;

        let response = self
            .http
            .post(&url, &self.key, &body)
            .await
            .map_err(|e| PaystackAPIError::TransactionSplit(e.to_string()))?;

        let parsed_response: Response<String> = serde_json::from_str(&response)
            .map_err(|e| PaystackAPIError::TransactionSplit(e.to_string()))?;

        Ok(parsed_response)
    }
}
