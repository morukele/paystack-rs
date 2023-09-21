//! Transactions
//! =============
//! The Transactions API allows you create and manage payments on your integration.

use crate::{
    get_request, post_request, ChargeBody, Currency, Error, ExportTransactionResponse,
    InitializeTransactionBody, PartialDebitTransactionBody, PaystackResult, Status,
    TransactionResponse, TransactionStatusListResponse, TransactionStatusResponse,
    TransactionTimelineResponse, TransactionTotalsResponse,
};
use reqwest::StatusCode;

/// A Struct to hold all the functions of the transaction API route
#[derive(Debug, Clone)]
pub struct TransactionEndpoints<'a> {
    /// Paystack API Key
    api_key: &'a str,
}

static BASE_URL: &str = "https://api.paystack.co";

impl<'a> TransactionEndpoints<'a> {
    /// Constructor for the transaction object
    pub fn new(key: &'a str) -> TransactionEndpoints<'a> {
        TransactionEndpoints { api_key: key }
    }

    /// Initialize a transaction from your backend.
    ///
    /// It takes a Transaction type as its parameter
    pub async fn initialize_transaction(
        &self,
        transaction_body: InitializeTransactionBody<'a>,
    ) -> PaystackResult<TransactionResponse> {
        let url = format!("{}/transaction/initialize", BASE_URL);

        match post_request(self.api_key, &url, transaction_body).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<TransactionResponse>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(Error::Transaction(err.to_string())),
                },
                _ => Err(Error::RequestNotSuccessful(
                    response.status().to_string(),
                    response.text().await?,
                )),
            },
            Err(err) => Err(Error::FailedRequest(err.to_string())),
        }
    }

    /// Confirm the status of a transaction.
    ///
    /// It takes the following parameters:
    ///     - reference: The transaction reference used to initiate the transaction
    pub async fn verify_transaction(
        &self,
        reference: &str,
    ) -> PaystackResult<TransactionStatusResponse> {
        let url = format!("{}/transaction/verify/{}", BASE_URL, reference);

        match get_request(self.api_key, &url, None).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<TransactionStatusResponse>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(Error::Transaction(err.to_string())),
                },
                _ => Err(Error::RequestNotSuccessful(
                    response.status().to_string(),
                    response.text().await?,
                )),
            },
            Err(err) => Err(Error::FailedRequest(err.to_string())),
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
    ) -> PaystackResult<TransactionStatusListResponse> {
        let url = format!("{}/transaction", BASE_URL);

        let per_page = number_of_transactions.unwrap_or(10).to_string();
        let status = status.unwrap_or(Status::Success).to_string();
        let query = vec![("perPage", per_page.as_str()), ("status", status.as_str())];

        match get_request(self.api_key, &url, Some(query)).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<TransactionStatusListResponse>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(Error::Transaction(err.to_string())),
                },
                _ => Err(Error::RequestNotSuccessful(
                    response.status().to_string(),
                    response.text().await?,
                )),
            },
            Err(err) => Err(Error::FailedRequest(err.to_string())),
        }
    }

    /// Get details of a transaction carried out on your integration.
    ///
    /// This methods take the Id of the desired transaction as a parameter
    pub async fn fetch_transactions(
        &self,
        transaction_id: u32,
    ) -> PaystackResult<TransactionStatusResponse> {
        let url = format!("{}/transaction/{}", BASE_URL, transaction_id);

        match get_request(self.api_key, &url, None).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<TransactionStatusResponse>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(Error::Transaction(err.to_string())),
                },
                _ => Err(Error::RequestNotSuccessful(
                    response.status().to_string(),
                    response.text().await?,
                )),
            },
            Err(err) => Err(Error::FailedRequest(err.to_string())),
        }
    }

    /// All authorizations marked as reusable can be charged with this endpoint whenever you need to receive payments.
    ///
    /// This function takes a Charge Struct as parameter
    pub async fn charge_authorization(
        &self,
        charge: ChargeBody<'a>,
    ) -> PaystackResult<TransactionStatusResponse> {
        let url = format!("{}/transaction/charge_authorization", BASE_URL);

        match post_request(self.api_key, &url, charge).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<TransactionStatusResponse>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(Error::Charge(err.to_string())),
                },
                _ => Err(Error::RequestNotSuccessful(
                    response.status().to_string(),
                    response.text().await?,
                )),
            },
            Err(err) => Err(Error::FailedRequest(err.to_string())),
        }
    }

    /// View the timeline of a transaction.
    ///
    /// This method takes in the Transaction id or reference as a parameter
    pub async fn view_transaction_timeline(
        &self,
        id: Option<u32>,
        reference: Option<&str>,
    ) -> PaystackResult<TransactionTimelineResponse> {
        // This is a hacky implementation to ensure that the transaction reference or id is not empty.
        // If they are empty, a url without them as parameter is created.
        let url: PaystackResult<String> = match (id, reference) {
            (Some(id), None) => Ok(format!("{}/transaction/timeline/{}", BASE_URL, id)),
            (None, Some(reference)) => {
                Ok(format!("{}/transaction/timeline/{}", BASE_URL, &reference))
            }
            _ => {
                return Err(Error::Transaction(
                    "Transaction Id or Reference is need to view transaction timeline".to_string(),
                ))
            }
        };

        let url = url.unwrap(); // Send the error back up the function

        match get_request(self.api_key, &url, None).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<TransactionTimelineResponse>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(Error::Transaction(err.to_string())),
                },
                _ => Err(Error::RequestNotSuccessful(
                    response.status().to_string(),
                    response.text().await?,
                )),
            },
            Err(err) => Err(Error::FailedRequest(err.to_string())),
        }
    }

    /// Total amount received on your account.
    ///
    /// This route normally takes a perPage or page query,
    /// However in this case it is ignored.
    /// If you need it in your work please open an issue
    /// and it will be implemented.
    pub async fn total_transactions(&self) -> PaystackResult<TransactionTotalsResponse> {
        let url = format!("{}/transaction/totals", BASE_URL);

        match get_request(self.api_key, &url, None).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<TransactionTotalsResponse>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(Error::Transaction(err.to_string())),
                },
                _ => Err(Error::RequestNotSuccessful(
                    response.status().to_string(),
                    response.text().await?,
                )),
            },
            Err(err) => Err(Error::FailedRequest(err.to_string())),
        }
    }

    /// Export a list of transactions carried out on your integration.
    ///
    /// This method takes the following parameters
    /// - Status (Optional): The status of the transactions to export. Defaults to all
    /// - Currency (Optional): The currency of the transactions to export. Defaults to NGN
    /// - Settled (Optional): To state of the transactions to export. Defaults to False.
    pub async fn export_transaction(
        &self,
        status: Option<Status>,
        currency: Option<Currency>,
        settled: Option<bool>,
    ) -> PaystackResult<ExportTransactionResponse> {
        let url = format!("{}/transaction/export", BASE_URL);

        // Specify a default option for settled transactions.
        let settled = match settled {
            Some(settled) => settled.to_string(),
            None => String::from(""),
        };

        let status = status.unwrap_or(Status::Success).to_string();
        let currency = currency.unwrap_or(Currency::EMPTY).to_string();

        let query = vec![
            ("status", status.as_str()),
            ("currency", currency.as_str()),
            ("settled", settled.as_str()),
        ];

        match get_request(self.api_key, &url, Some(query)).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<ExportTransactionResponse>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(Error::Transaction(err.to_string())),
                },
                _ => Err(Error::RequestNotSuccessful(
                    response.status().to_string(),
                    response.text().await?,
                )),
            },
            Err(err) => Err(Error::FailedRequest(err.to_string())),
        }
    }

    /// Retrieve part of a payment from a customer.
    ///
    /// It takes a PartialDebitTransaction type as a parameter.
    ///
    /// NB: it must be created with the PartialDebitTransaction Builder.
    pub async fn partial_debit(
        &self,
        transaction_body: PartialDebitTransactionBody<'a>,
    ) -> PaystackResult<TransactionStatusResponse> {
        let url = format!("{}/transaction/partial_debit", BASE_URL);

        match post_request(self.api_key, &url, transaction_body).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<TransactionStatusResponse>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(Error::Transaction(err.to_string())),
                },
                _ => Err(Error::RequestNotSuccessful(
                    response.status().to_string(),
                    response.text().await?,
                )),
            },
            Err(err) => Err(Error::FailedRequest(err.to_string())),
        }
    }
}
