//! Transaction Split
//! =================
//! The Transaction Splits API enables merchants split the settlement for a
//! transaction across their payout account, and one or more subaccounts.

use crate::{
    HttpClient, PaystackAPIError, PaystackResult, Response, TransactionSplitRequest,
    TransactionSplitResponseData,
};
use std::sync::Arc;

/// A struct to hold all the functions of the transaction split API endpoint
#[derive(Debug, Clone)]
pub struct TransactionSplitEndpoints<T: HttpClient + Default> {
    key: String,
    base_url: String,
    http: Arc<T>,
}

impl<T: HttpClient + Default> TransactionSplitEndpoints<T> {
    /// Constructor for the transaction object
    pub fn new(key: String, http: Arc<T>) -> TransactionSplitEndpoints<T> {
        let base_url = String::from("https://api.paystack.co/split");
        TransactionSplitEndpoints {
            key,
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
}
