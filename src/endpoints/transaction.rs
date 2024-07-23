//! Transactions
//! =============
//! The Transaction route allows to create and manage payments on your integration.

use crate::{
    ChargeRequest, HttpClient, PaystackAPIError, PaystackResult, Response, Status,
    TransactionRequest, TransactionResponseData, TransactionStatusData, TransactionTimelineData,
    TransactionTotalData,
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
                Err(PaystackAPIError::Transaction(e.to_string()))
            }
        }
    }

    /// Confirm the status of a transaction.
    ///
    /// It takes the following parameters:
    ///     - reference: The transaction reference used to initiate the transaction
    pub async fn verify_transaction(
        &self,
        reference: &str,
    ) -> PaystackResult<TransactionStatusData> {
        let url = format!("{}/verify/{}", self.base_url, reference);

        let response = self.http.get(&url, &self.key, None).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<TransactionStatusData> =
                    serde_json::from_str(&response)
                        .map_err(|e| PaystackAPIError::Transaction(e.to_string()))?;

                Ok(parsed_response)
            }
            Err(e) => Err(PaystackAPIError::Transaction(e.to_string())),
        }
    }

    /// List transactions carried out on your integration.
    ///
    /// The method takes the following parameters:
    ///     - perPage (Optional): Number of transactions to return. If None is passed as the parameter, the last 10 transactions are returned.
    ///     - status (Optional): Filter transactions by status, defaults to Success if no status is passed.
    ///
    pub async fn list_transactions(
        &self,
        number_of_transactions: Option<u32>,
        status: Option<Status>,
    ) -> PaystackResult<Vec<TransactionStatusData>> {
        let url = self.base_url.to_string();

        let per_page = number_of_transactions.unwrap_or(10).to_string();
        let status = status.unwrap_or(Status::Success).to_string();
        let query = vec![("perPage", per_page.as_str()), ("status", status.as_str())];

        let response = self.http.get(&url, &self.key, Some(&query)).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<Vec<TransactionStatusData>> =
                    serde_json::from_str(&response)
                        .map_err(|e| PaystackAPIError::Transaction(e.to_string()))?;

                Ok(parsed_response)
            }
            Err(e) => Err(PaystackAPIError::Transaction(e.to_string())),
        }
    }

    /// Get details of a transaction carried out on your integration.
    ///
    /// This method take the ID of the desired transaction as a parameter
    pub async fn fetch_transactions(
        &self,
        transaction_id: u32,
    ) -> PaystackResult<TransactionStatusData> {
        let url = format!("{}/{}", self.base_url, transaction_id);

        let response = self.http.get(&url, &self.key, None).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<TransactionStatusData> =
                    serde_json::from_str(&response)
                        .map_err(|e| PaystackAPIError::Transaction(e.to_string()))?;

                Ok(parsed_response)
            }
            Err(e) => Err(PaystackAPIError::Transaction(e.to_string())),
        }
    }

    /// All authorizations marked as reusable can be charged with this endpoint whenever you need to receive payments.
    ///
    /// This function takes a Charge Struct as parameter
    pub async fn charge_authorization(
        &self,
        charge_request: ChargeRequest,
    ) -> PaystackResult<TransactionStatusData> {
        let url = format!("{}/charge_authorization", self.base_url);
        let body = serde_json::to_value(charge_request)
            .map_err(|e| PaystackAPIError::Transaction(e.to_string()))?;

        let response = self.http.post(&url, &self.key, &body).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<TransactionStatusData> =
                    serde_json::from_str(&response)
                        .map_err(|e| PaystackAPIError::Transaction(e.to_string()))?;

                Ok(parsed_response)
            }
            Err(e) => Err(PaystackAPIError::Transaction(e.to_string())),
        }
    }

    /// View the timeline of a transaction.
    ///
    /// This method takes in the Transaction id or reference as a parameter
    pub async fn view_transaction_timeline(
        &self,
        id: Option<u32>,
        reference: Option<&str>,
    ) -> PaystackResult<TransactionTimelineData> {
        // This is a hacky implementation to ensure that the transaction reference or id is not empty.
        // If they are empty, a new url without them as parameter is created.
        let url = match (id, reference) {
            (Some(id), None) => Ok(format!("{}/timeline/{}", self.base_url, id)),
            (None, Some(reference)) => Ok(format!("{}/timeline/{}", self.base_url, &reference)),
            _ => Err(PaystackAPIError::Transaction(
                "Transaction Id or Reference is need to view transaction timeline".to_string(),
            )),
        }?; // propagate the error upstream

        let response = self.http.get(&url, &self.key, None).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<TransactionTimelineData> =
                    serde_json::from_str(&response)
                        .map_err(|e| PaystackAPIError::Transaction(e.to_string()))?;

                Ok(parsed_response)
            }
            Err(e) => Err(PaystackAPIError::Transaction(e.to_string())),
        }
    }

    /// Total amount received on your account.
    ///
    /// This route normally takes a perPage or page query,
    /// However in this case it is ignored.
    /// If you need it in your work please open an issue
    /// and it will be implemented.
    pub async fn total_transactions(&self) -> PaystackResult<TransactionTotalData> {
        let url = format!("{}/totals", self.base_url);

        let response = self.http.get(&url, &self.key, None).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<TransactionTotalData> = serde_json::from_str(&response)
                    .map_err(|e| PaystackAPIError::Transaction(e.to_string()))?;

                Ok(parsed_response)
            }
            Err(e) => Err(PaystackAPIError::Transaction(e.to_string())),
        }
    }
}
