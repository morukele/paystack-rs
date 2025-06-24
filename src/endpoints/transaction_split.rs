//! Transaction Split
//! =================
//! The Transaction Splits API enables merchants split the settlement for a
//! transaction across their payout account, and one or more subaccounts.

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
    /// Constructor
    pub fn new(key: Arc<String>, http: Arc<T>) -> TransactionSplitEndpoints<T> {
        let base_url = String::from("https://api.paystack.co/split");
        TransactionSplitEndpoints {
            key: key.to_string(),
            base_url,
            http,
        }
    }

    /// Create a split payment on your integration.
    ///
    /// This method takes a `TransactionSplitRequest` object as a parameter.
    pub async fn create_transaction_split(
        &self,
        split_body: TransactionSplitRequest,
    ) -> PaystackResult<TransactionSplitResponseData> {
        let url = self.base_url.to_string();
        let body = serde_json::to_value(split_body)
            .map_err(|e| PaystackAPIError::TransactionSplit(e.to_string()))?;

        let response = self.http.post(&url, &self.key, &body).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<TransactionSplitResponseData> =
                    serde_json::from_str(&response)
                        .map_err(|e| PaystackAPIError::TransactionSplit(e.to_string()))?;
                Ok(parsed_response)
            }
            Err(e) => Err(PaystackAPIError::TransactionSplit(e.to_string())),
        }
    }

    /// List the transaction splits available on your integration
    ///
    /// Takes in the following parameters:
    ///     - `split_name`: (Optional) name of the split to retrieve.
    ///     - `split_active`: (Optional) status of the split to retrieve.
    pub async fn list_transaction_splits(
        &self,
        split_name: Option<&str>,
        split_active: Option<bool>,
    ) -> PaystackResult<Vec<TransactionSplitResponseData>> {
        let url = self.base_url.to_string();

        // Specify a default option for active splits
        let split_active = match split_active {
            Some(active) => active.to_string(),
            None => "".to_string(),
        };

        let query = vec![
            ("name", split_name.unwrap_or("")),
            ("active", &split_active),
        ];

        let response = self.http.get(&url, &self.key, Some(&query)).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<Vec<TransactionSplitResponseData>> =
                    serde_json::from_str(&response)
                        .map_err(|e| PaystackAPIError::TransactionSplit(e.to_string()))?;

                Ok(parsed_response)
            }
            Err(e) => Err(PaystackAPIError::TransactionSplit(e.to_string())),
        }
    }

    /// Get details of a split on your integration.
    ///
    /// Takes in the following parameter:
    ///     - `split_id`:  ID of the transaction split.
    pub async fn fetch_transaction_split(
        &self,
        split_id: &str,
    ) -> PaystackResult<TransactionSplitResponseData> {
        let url = format!("{}/{}", self.base_url, split_id);

        let response = self.http.get(&url, &self.key, None).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<TransactionSplitResponseData> =
                    serde_json::from_str(&response)
                        .map_err(|e| PaystackAPIError::TransactionSplit(e.to_string()))?;

                Ok(parsed_response)
            }
            Err(e) => Err(PaystackAPIError::TransactionSplit(e.to_string())),
        }
    }

    /// Update a transaction split details on your integration.
    ///
    /// Takes in a
    /// - `update_body` as a `UpdateTransactionSplitRequest` struct which is created from the `UpdateTransactionSplitRequestBuilder` struct
    /// - `split_id`, the ID of the split to update
    pub async fn update_transaction_split(
        &self,
        split_id: &str,
        update_body: UpdateTransactionSplitRequest,
    ) -> PaystackResult<TransactionSplitResponseData> {
        let url = format!("{}/{}", self.base_url, split_id);
        let body = serde_json::to_value(update_body)
            .map_err(|e| PaystackAPIError::TransactionSplit(e.to_string()))?;

        let response = self.http.put(&url, &self.key, &body).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<TransactionSplitResponseData> =
                    serde_json::from_str(&response)
                        .map_err(|e| PaystackAPIError::TransactionSplit(e.to_string()))?;
                Ok(parsed_response)
            }
            Err(e) => Err(PaystackAPIError::TransactionSplit(e.to_string())),
        }
    }

    /// Add a Subaccount to a Transaction Split, or update the share of an existing Subaccount in a Transaction Split
    ///
    /// Takes in the following parameters:
    ///     - `split_id`: Id of the transaction split to update.
    ///     - `body`: Subaccount to add to the transaction split.
    pub async fn add_or_update_subaccount_split(
        &self,
        split_id: &str,
        body: SubaccountBody,
    ) -> PaystackResult<TransactionSplitResponseData> {
        let url = format!("{}/{}/subaccount/add", self.base_url, split_id);
        let body = serde_json::to_value(body)
            .map_err(|e| PaystackAPIError::TransactionSplit(e.to_string()))?;

        let response = self.http.post(&url, &self.key, &body).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<TransactionSplitResponseData> =
                    serde_json::from_str(&response)
                        .map_err(|e| PaystackAPIError::TransactionSplit(e.to_string()))?;

                Ok(parsed_response)
            }
            Err(e) => Err(PaystackAPIError::TransactionSplit(e.to_string())),
        }
    }

    /// Remove a subaccount from a transaction split.
    ///
    /// Takes in the following parameters
    ///     - split_id: Id of the transaction split
    ///     - subaccount: subaccount code to remove
    pub async fn remove_subaccount_from_transaction_split(
        &self,
        split_id: &str,
        subaccount: DeleteSubAccountBody,
    ) -> PaystackResult<String> {
        let url = format!("{}/{}/subaccount/remove", self.base_url, split_id);
        let body = serde_json::to_value(subaccount)
            .map_err(|e| PaystackAPIError::TransactionSplit(e.to_string()))?;

        let response = self.http.post(&url, &self.key, &body).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<String> = serde_json::from_str(&response)
                    .map_err(|e| PaystackAPIError::TransactionSplit(e.to_string()))?;

                Ok(parsed_response)
            }
            Err(e) => Err(PaystackAPIError::TransactionSplit(e.to_string())),
        }
    }
}
