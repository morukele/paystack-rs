//! Transactions
//! =============
//! The Transaction route allows to create and manage payments on your integration.

use crate::{
    HttpClient, PaystackAPIError, PaystackResult, Response, TransactionRequest,
    TransactionResponseData,
};
use std::marker::PhantomData;
use std::sync::Arc;

/// A struct to hold all the functions of the transaction API endpoint
#[derive(Debug, Clone)]
pub struct TransactionEndpoints<'a, T: HttpClient + Default> {
    /// Paystack API Key
    key: String,
    /// Base URL for the transaction route
    base_url: String,
    /// Http client for the route
    http: Arc<T>,

    // to keep compiler happy
    phantom: PhantomData<&'a T>,
}

impl<'a, T: HttpClient + Default> TransactionEndpoints<'a, T> {
    /// Constructor for the transaction object
    pub fn new(key: String, http: Arc<T>) -> TransactionEndpoints<'a, T> {
        let base_url = String::from("https://api.paystack.co/transaction");
        TransactionEndpoints {
            key,
            base_url,
            http,
            // useless
            phantom: PhantomData,
        }
    }

    /// Initialize a transaction in your integration
    ///
    /// Takes a `TransactionRequest`struct as input.
    pub async fn initialize_transaction(
        &self,
        transaction_request: TransactionRequest,
    ) -> PaystackResult<TransactionResponseData> {
        let url = format!("{}/initialize", self.base_url);
        let body = serde_json::to_value(transaction_request)
            .map_err(|e| PaystackAPIError::Transaction(e.to_string()))?;

        let response = self.http.post(&url, &self.key, &body).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<TransactionResponseData> =
                    serde_json::from_str(&response)
                        .map_err(|e| PaystackAPIError::Transaction(e.to_string()))?;
                Ok(parsed_response)
            }
            Err(e) => {
                // convert the error to a transaction error
                Err(PaystackAPIError::Transaction(
                    e.to_string(),
                ))
            }
        }
    }
}
