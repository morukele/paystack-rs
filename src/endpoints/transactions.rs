//! Transactions
//! =============
//! The Transaction route allows to create and manage payments on your integration.

use super::PAYSTACK_BASE_URL;
use crate::{
    ChargeRequest, ChargeResponseData, Currency, ExportTransactionData, HttpClient,
    PartialDebitTransactionRequest, PaystackAPIError, PaystackResult, Response, Status,
    TransactionIdentifier, TransactionRequest, TransactionResponseData, TransactionStatusData,
    TransactionTimelineData, TransactionTotalData,
};
use std::sync::Arc;

/// A struct to hold all the functions of the transaction API endpoint
#[derive(Debug, Clone)]
pub struct TransactionEndpoints<T: HttpClient + Default> {
    /// Paystack API Key
    key: String,
    /// Base URL for the transaction route
    base_url: String,
    /// Http client for the route
    http: Arc<T>,
}

impl<T: HttpClient + Default> TransactionEndpoints<T> {
    /// Creates a new TransactionEndpoints instance
    ///
    /// # Arguments
    /// * `key` - The Paystack API key
    /// * `http` - The HTTP client implementation to use for API requests
    ///
    /// # Returns
    /// A new TransactionEndpoints instance
    pub fn new(key: Arc<String>, http: Arc<T>) -> TransactionEndpoints<T> {
        let base_url = format!("{PAYSTACK_BASE_URL}/transaction");
        TransactionEndpoints {
            key: key.to_string(),
            base_url,
            http,
        }
    }

    /// Initialize a transaction in your integration
    ///
    /// # Arguments
    /// * `transaction_request` - The request data to initialize the transaction.
    ///   Should be created with a `TransactionRequestBuilder` struct
    ///
    /// # Returns
    /// A Result containing the transaction response data or an error
    pub async fn initialize_transaction(
        &self,
        transaction_request: TransactionRequest,
    ) -> PaystackResult<TransactionResponseData> {
        let url = format!("{}/initialize", self.base_url);
        let body = serde_json::to_value(transaction_request)
            .map_err(|e| PaystackAPIError::Transaction(e.to_string()))?;

        let response = self
            .http
            .post(&url, &self.key, &body)
            .await
            .map_err(|e| PaystackAPIError::Transaction(e.to_string()))?;

        let parsed_response: Response<TransactionResponseData> = serde_json::from_str(&response)
            .map_err(|e| PaystackAPIError::Transaction(e.to_string()))?;
        Ok(parsed_response)
    }

    /// Verifies the status of a transaction
    ///
    /// # Arguments
    /// * `reference` - The transaction reference used to initiate the transaction
    ///
    /// # Returns
    /// A Result containing the transaction status data or an error
    pub async fn verify_transaction(
        &self,
        reference: &str,
    ) -> PaystackResult<TransactionStatusData> {
        let url = format!("{}/verify/{}", self.base_url, reference);

        let response = self
            .http
            .get(&url, &self.key, None)
            .await
            .map_err(|e| PaystackAPIError::Transaction(e.to_string()))?;

        let parsed_response: Response<TransactionStatusData> = serde_json::from_str(&response)
            .map_err(|e| PaystackAPIError::Transaction(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Lists transactions carried out on your integration
    ///
    /// # Arguments
    /// * `per_page` - Optional number of transactions to return per page. Defaults to 10 if None
    /// * `status` - Optional filter for transaction status. Defaults to Success if None
    ///
    /// # Returns
    /// A Result containing a vector of transaction status data or an error
    pub async fn list_transactions(
        &self,
        per_page: Option<u32>,
        status: Option<Status>,
    ) -> PaystackResult<Vec<TransactionStatusData>> {
        let url = &self.base_url;

        let per_page = per_page.unwrap_or(10).to_string();
        let status = status.unwrap_or(Status::Success).to_string();
        let query = vec![("perPage", per_page.as_str()), ("status", status.as_str())];

        let response = self
            .http
            .get(url, &self.key, Some(&query))
            .await
            .map_err(|e| PaystackAPIError::Transaction(e.to_string()))?;

        let parsed_response: Response<Vec<TransactionStatusData>> = serde_json::from_str(&response)
            .map_err(|e| PaystackAPIError::Transaction(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Gets details of a specific transaction
    ///
    /// # Arguments
    /// * `transaction_id` - The ID of the transaction to fetch
    ///
    /// # Returns
    /// A Result containing the transaction status data or an error
    pub async fn fetch_transactions(
        &self,
        transaction_id: u64,
    ) -> PaystackResult<TransactionStatusData> {
        let url = format!("{}/{}", self.base_url, transaction_id);

        let response = self
            .http
            .get(&url, &self.key, None)
            .await
            .map_err(|e| PaystackAPIError::Transaction(e.to_string()))?;

        let parsed_response: Response<TransactionStatusData> = serde_json::from_str(&response)
            .map_err(|e| PaystackAPIError::Transaction(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Charges a reusable authorization
    ///
    /// # Arguments
    /// * `charge_request` - The charge request data containing authorization details.
    ///   Should be created with the `ChargeRequestBuilder` struct.
    ///
    /// # Returns
    /// A Result containing the charge response data or an error
    pub async fn charge_authorization(
        &self,
        charge_request: ChargeRequest,
    ) -> PaystackResult<ChargeResponseData> {
        let url = format!("{}/charge_authorization", self.base_url);
        let body = serde_json::to_value(charge_request)
            .map_err(|e| PaystackAPIError::Transaction(e.to_string()))?;

        let response = self
            .http
            .post(&url, &self.key, &body)
            .await
            .map_err(|e| PaystackAPIError::Transaction(e.to_string()))?;

        let parsed_response: Response<ChargeResponseData> =
            serde_json::from_str(&response).map_err(|e| PaystackAPIError::Charge(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Views the timeline of a transaction
    ///
    /// # Arguments
    /// * `identifier` - The transaction identifier (either ID or reference)
    ///
    /// # Returns
    /// A Result containing the transaction timeline data or an error
    pub async fn view_transaction_timeline(
        &self,
        identifier: TransactionIdentifier,
    ) -> PaystackResult<TransactionTimelineData> {
        // This is a hacky implementation to ensure that the transaction reference or id is not empty.
        // If they are empty, a new url without them as parameter is created.
        let url = match identifier {
            TransactionIdentifier::Id(id) => Ok(format!("{}/timeline/{}", self.base_url, id)),
            TransactionIdentifier::Reference(reference) => {
                Ok(format!("{}/timeline/{}", self.base_url, &reference))
            }
        }?; // propagate the error upstream

        let response = self
            .http
            .get(&url, &self.key, None)
            .await
            .map_err(|e| PaystackAPIError::Transaction(e.to_string()))?;

        let parsed_response: Response<TransactionTimelineData> = serde_json::from_str(&response)
            .map_err(|e| PaystackAPIError::Transaction(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Gets the total amount received on your account
    ///
    /// # Returns
    /// A Result containing the transaction total data or an error
    pub async fn total_transactions(&self) -> PaystackResult<TransactionTotalData> {
        let url = format!("{}/totals", self.base_url);

        let response = self
            .http
            .get(&url, &self.key, None)
            .await
            .map_err(|e| PaystackAPIError::Transaction(e.to_string()))?;

        let parsed_response: Response<TransactionTotalData> = serde_json::from_str(&response)
            .map_err(|e| PaystackAPIError::Transaction(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Exports a list of transactions
    ///
    /// # Arguments
    /// * `status` - Optional status filter for transactions to export. Defaults to Success
    /// * `currency` - Optional currency filter. Defaults to NGN
    /// * `settled` - Optional filter for settled transactions. Defaults to false
    ///
    /// # Returns
    /// A Result containing the export transaction data or an error
    pub async fn export_transaction(
        &self,
        status: Option<Status>,
        currency: Option<Currency>,
        settled: Option<bool>,
    ) -> PaystackResult<ExportTransactionData> {
        let url = format!("{}/export", self.base_url);

        // Specify a default option for settled transactions.
        let settled = match settled {
            Some(settled) => settled.to_string(),
            None => String::from(""),
        };

        let status = status.unwrap_or(Status::Success).to_string();
        let currency = currency.unwrap_or(Currency::NGN).to_string();

        let query = vec![
            ("status", status.as_str()),
            ("currency", currency.as_str()),
            ("settled", settled.as_str()),
        ];

        let response = self
            .http
            .get(&url, &self.key, Some(&query))
            .await
            .map_err(|e| PaystackAPIError::Transaction(e.to_string()))?;

        let parsed_response = serde_json::from_str(&response)
            .map_err(|e| PaystackAPIError::Transaction(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Performs a partial debit on a transaction
    ///
    /// # Arguments
    /// * `partial_debit_transaction_request` - The request data for the partial debit.
    ///   Must be created with the `PartialDebitTransactionBuilder` Struct.
    ///
    /// # Returns
    /// A Result containing the transaction status data or an error
    pub async fn partial_debit(
        &self,
        partial_debit_transaction_request: PartialDebitTransactionRequest,
    ) -> PaystackResult<TransactionStatusData> {
        let url = format!("{}/partial_debit", self.base_url);
        let body = serde_json::to_value(partial_debit_transaction_request)
            .map_err(|e| PaystackAPIError::Transaction(e.to_string()))?;

        let response = self
            .http
            .post(&url, &self.key, &body)
            .await
            .map_err(|e| PaystackAPIError::Transaction(e.to_string()))?;

        let parsed_response: Response<TransactionStatusData> = serde_json::from_str(&response)
            .map_err(|e| PaystackAPIError::Transaction(e.to_string()))?;

        Ok(parsed_response)
    }
}
