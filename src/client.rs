//! Client
//! ===========
//! The file for the Paystack API client and it's associated functions

extern crate reqwest;
extern crate serde_json;

use reqwest::{Response, StatusCode};
use serde::Serialize;
use std::fmt::Debug;

use crate::{
    Charge, Currency, ExportTransactionResponse, PartialDebitTransaction, PaystackError,
    PaystackResult, RequestNotSuccessful, ResponseWithoutData, Status, Subaccount, Transaction,
    TransactionResponse, TransactionSplit, TransactionSplitListResponse, TransactionSplitResponse,
    TransactionStatus, TransactionStatusList, TransactionTimeline, TransactionTotalsResponse,
};

static BASE_URL: &str = "https://api.paystack.co";

/// This is the struct that allows you to authenticate to the PayStack API.
/// It contains the API key which allows you to interact with the API.
#[derive(Clone, Debug)]
pub struct PaystackClient {
    client: reqwest::Client,
    api_key: String,
}

impl PaystackClient {
    /// This method creates a new PayStack client with the specified API key.
    ///
    /// It takes the following parameters:
    ///     - key: Paystack API key.
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key: key.into(),
        }
    }

    /// A function for sending GET request to a specified url
    /// with optional query parameters using reqwest client.
    async fn get_request(
        &self,
        url: &String,
        query: Option<Vec<(&str, String)>>,
    ) -> PaystackResult<Response> {
        let response = self
            .client
            .get(url)
            .query(&query)
            .bearer_auth(&self.api_key)
            .header("Content-Type", "application/json")
            .send()
            .await;

        match response {
            Ok(response) => match response.status() {
                StatusCode::OK => Ok(response),
                _ => {
                    Err(RequestNotSuccessful::new(response.status(), response.text().await?).into())
                }
            },
            Err(err) => Err(PaystackError::Generic(err.to_string())),
        }
    }

    /// A function for sending POST requests to a specified url
    /// using the reqwest client.
    async fn post_request<T>(&self, url: &String, body: T) -> PaystackResult<Response>
    where
        T: Debug + Serialize,
    {
        let response = self
            .client
            .post(url)
            .bearer_auth(&self.api_key)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await;

        match response {
            Ok(response) => match response.status() {
                StatusCode::OK => Ok(response),
                _ => {
                    Err(RequestNotSuccessful::new(response.status(), response.text().await?).into())
                }
            },
            Err(err) => Err(PaystackError::Generic(err.to_string())),
        }
    }

    /// A function for sending PUT requests to a specified url
    /// using the reqwest client.
    async fn put_request<T>(&self, url: &String, body: T) -> PaystackResult<Response>
    where
        T: Debug + Serialize,
    {
        let response = self
            .client
            .put(url)
            .bearer_auth(&self.api_key)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await;

        match response {
            Ok(response) => match response.status() {
                StatusCode::OK => Ok(response),
                _ => {
                    Err(RequestNotSuccessful::new(response.status(), response.text().await?).into())
                }
            },
            Err(err) => Err(PaystackError::Generic(err.to_string())),
        }
    }

    /// A function for sending DELETE requests to a specified url
    /// using the reqwest client.
    async fn _delete_request<T>(&self, url: &String, body: T) -> PaystackResult<Response>
    where
        T: Debug + Serialize,
    {
        let response = self
            .client
            .delete(url)
            .bearer_auth(&self.api_key)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await;

        match response {
            Ok(response) => match response.status() {
                StatusCode::OK => Ok(response),
                _ => {
                    Err(RequestNotSuccessful::new(response.status(), response.text().await?).into())
                }
            },
            Err(err) => Err(PaystackError::Generic(err.to_string())),
        }
    }

    /// This method initalizes a new transaction using the Paystack API.
    ///
    /// It takes a Transaction type as its parameter
    pub async fn initialize_transaction(
        &self,
        transaction_body: Transaction,
    ) -> PaystackResult<TransactionResponse> {
        let url = format!("{}/transaction/initialize", BASE_URL);

        match self.post_request(&url, transaction_body).await {
            Ok(response) => match response.status() {
                reqwest::StatusCode::OK => match response.json::<TransactionResponse>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(PaystackError::Transaction(err.to_string())),
                },
                _ => {
                    Err(RequestNotSuccessful::new(response.status(), response.text().await?).into())
                }
            },
            Err(err) => Err(err),
        }
    }

    /// This method confirms the status of a transaction.
    ///
    /// It takes the following parameters:
    ///     - reference: The transaction reference used to intiate the transaction
    pub async fn verify_transaction(&self, reference: String) -> PaystackResult<TransactionStatus> {
        let url = format!("{}/transaction/verify/{}", BASE_URL, reference);

        match self.get_request(&url, None).await {
            Ok(response) => match response.status() {
                reqwest::StatusCode::OK => match response.json::<TransactionStatus>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(PaystackError::Transaction(err.to_string())),
                },
                _ => {
                    Err(RequestNotSuccessful::new(response.status(), response.text().await?).into())
                }
            },
            Err(err) => Err(err),
        }
    }

    /// This method returns a Vec of transactions carried out on your integrations.
    ///
    /// The method takes the following parameters:
    ///     - perPage (Optional): Number of transactions to return. If None is passed as the parameter, the last 10 transactions are returned.
    ///     - status (Optional): Filter transactions by status, defaults to Success if no status is passed.
    ///
    pub async fn list_transactions(
        &self,
        number_of_transactions: Option<u32>,
        status: Option<Status>,
    ) -> PaystackResult<TransactionStatusList> {
        let url = format!("{}/transaction", BASE_URL);
        let query = vec![
            ("perPage", number_of_transactions.unwrap_or(10).to_string()),
            ("status", status.unwrap_or(Status::Success).to_string()),
        ];

        match self.get_request(&url, Some(query)).await {
            Ok(response) => match response.status() {
                reqwest::StatusCode::OK => match response.json::<TransactionStatusList>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(PaystackError::Transaction(err.to_string())),
                },
                _ => {
                    Err(RequestNotSuccessful::new(response.status(), response.text().await?).into())
                }
            },
            Err(err) => Err(err),
        }
    }

    /// Get details of a transaction carried out on your integration
    ///
    /// This methods take the Id of the desired transaction as a parameter
    pub async fn fetch_transactions(
        &self,
        transaction_id: u32,
    ) -> PaystackResult<TransactionStatus> {
        let url = format!("{}/transaction/{}", BASE_URL, transaction_id);

        match self.get_request(&url, None).await {
            Ok(response) => match response.status() {
                reqwest::StatusCode::OK => match response.json::<TransactionStatus>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(PaystackError::Transaction(err.to_string())),
                },
                _ => {
                    Err(RequestNotSuccessful::new(response.status(), response.text().await?).into())
                }
            },
            Err(err) => Err(err),
        }
    }

    /// All authorizations marked as reusable can be charged with this endpoint whenever you need to receive payments
    ///
    /// This function takes a Charge Struct as parameter
    pub async fn charge_authorization(&self, charge: Charge) -> PaystackResult<TransactionStatus> {
        let url = format!("{}/transaction/charge_authorization", BASE_URL);

        match self.post_request(&url, charge).await {
            Ok(response) => match response.status() {
                reqwest::StatusCode::OK => match response.json::<TransactionStatus>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(PaystackError::Charge(err.to_string())),
                },
                _ => {
                    Err(RequestNotSuccessful::new(response.status(), response.text().await?).into())
                }
            },
            Err(err) => Err(err),
        }
    }

    /// View the timeline of a transaction
    ///
    /// This method takes in the Transaction id or reference as a parameter
    pub async fn view_transaction_timeline(
        &self,
        id: Option<u32>,
        reference: Option<String>,
    ) -> PaystackResult<TransactionTimeline> {
        // This is a hacky implementation to ensure that the transaction reference or id is not empty.
        // If they are empyt, a url without them as parameter is created.
        let url: PaystackResult<String> = match (id, reference) {
            (Some(id), None) => Ok(format!("{}/transaction/timeline/{}", BASE_URL, id)),
            (None, Some(reference)) => {
                Ok(format!("{}/transaction/timeline/{}", BASE_URL, &reference))
            }
            _ => {
                return Err(PaystackError::Transaction(
                    "Transaction Id or Reference is need to view transaction timeline".to_string(),
                ))
            }
        };

        let url = url.unwrap(); // Send the error back up the function

        match self.get_request(&url, None).await {
            Ok(response) => match response.status() {
                reqwest::StatusCode::OK => match response.json::<TransactionTimeline>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(PaystackError::Transaction(err.to_string())),
                },
                _ => {
                    Err(RequestNotSuccessful::new(response.status(), response.text().await?).into())
                }
            },
            Err(err) => Err(err),
        }
    }

    /// Total amount received on your account.
    ///
    /// This route normally takes a perPage or page query,
    /// However in this case it is ignrored.
    /// If you need it in your work please open an issue
    /// and it will be implemented.
    pub async fn total_transactions(&self) -> PaystackResult<TransactionTotalsResponse> {
        let url = format!("{}/transaction/totals", BASE_URL);

        match self.get_request(&url, None).await {
            Ok(response) => match response.status() {
                reqwest::StatusCode::OK => match response.json::<TransactionTotalsResponse>().await
                {
                    Ok(content) => Ok(content),
                    Err(err) => Err(PaystackError::Transaction(err.to_string())),
                },
                _ => {
                    Err(RequestNotSuccessful::new(response.status(), response.text().await?).into())
                }
            },
            Err(err) => Err(err),
        }
    }

    /// Export a list of transactions carried out on your integration
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

        let query = vec![
            ("status", status.unwrap_or(Status::Success).to_string()),
            ("currency", currency.unwrap_or(Currency::EMPTY).to_string()),
            ("settled", settled),
        ];

        match self.get_request(&url, Some(query)).await {
            Ok(response) => match response.status() {
                reqwest::StatusCode::OK => match response.json::<ExportTransactionResponse>().await
                {
                    Ok(content) => Ok(content),
                    Err(err) => Err(PaystackError::Transaction(err.to_string())),
                },
                _ => {
                    Err(RequestNotSuccessful::new(response.status(), response.text().await?).into())
                }
            },
            Err(err) => Err(err),
        }
    }

    /// Retrieve part of a payment from a customer.
    ///
    /// It takes a PartialDebitTransaction type as a parameter.
    ///
    /// NB: it must be created with the PartialDebitTransaction Builder.
    pub async fn partial_debit(
        &self,
        transaction_body: PartialDebitTransaction,
    ) -> PaystackResult<TransactionStatus> {
        let url = format!("{}/transaction/partial_debit", BASE_URL);

        match self.post_request(&url, transaction_body).await {
            Ok(response) => match response.status() {
                reqwest::StatusCode::OK => match response.json::<TransactionStatus>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(PaystackError::Transaction(err.to_string())),
                },
                _ => {
                    Err(RequestNotSuccessful::new(response.status(), response.text().await?).into())
                }
            },
            Err(err) => Err(err),
        }
    }

    /// Create a split payment on your integration.
    ///
    /// This method takes a TransactionSplit object as a parameter.
    pub async fn create_transaction_split(
        &self,
        split_body: TransactionSplit,
    ) -> PaystackResult<TransactionSplitResponse> {
        let url = format!("{}/split", BASE_URL);

        match self.post_request(&url, split_body).await {
            Ok(response) => match response.status() {
                reqwest::StatusCode::OK => {
                    match response.json::<TransactionSplitResponse>().await {
                        Ok(content) => Ok(content),
                        Err(err) => Err(PaystackError::TransactionSplit(err.to_string())),
                    }
                }
                _ => {
                    Err(RequestNotSuccessful::new(response.status(), response.text().await?).into())
                }
            },
            Err(err) => Err(err),
        }
    }

    /// List the transaction splits available on your integration
    ///
    /// Takes in the following parameters:
    ///     - `split_name`: (Optional) name of the split to retrieve.
    ///     - `split_active`: (Optional) status of the split to retrieve.
    pub async fn list_transaction_splits(
        &self,
        split_name: Option<String>,
        split_active: Option<bool>,
    ) -> PaystackResult<TransactionSplitListResponse> {
        let url = format!("{}/split", BASE_URL);

        // Specify a default option for active splits
        let split_active = match split_active {
            Some(active) => active.to_string(),
            None => String::from(""),
        };

        let query = vec![
            ("name", split_name.unwrap_or("".to_string())),
            ("active", split_active),
        ];

        match self.get_request(&url, Some(query)).await {
            Ok(response) => match response.status() {
                reqwest::StatusCode::OK => {
                    match response.json::<TransactionSplitListResponse>().await {
                        Ok(content) => Ok(content),
                        Err(err) => Err(PaystackError::TransactionSplit(err.to_string())),
                    }
                }
                _ => {
                    Err(RequestNotSuccessful::new(response.status(), response.text().await?).into())
                }
            },
            Err(err) => Err(err),
        }
    }

    /// Get details of a split on your integration.
    ///
    /// Takes in the following parameter:
    ///     - `split_id`:  Id of the transaction split.
    pub async fn fetch_transaction_split(
        &self,
        split_id: String,
    ) -> PaystackResult<TransactionSplitResponse> {
        let url = format!("{}/split{}", BASE_URL, split_id);

        match self.get_request(&url, None).await {
            Ok(response) => match response.status() {
                reqwest::StatusCode::OK => {
                    match response.json::<TransactionSplitResponse>().await {
                        Ok(content) => Ok(content),
                        Err(err) => Err(PaystackError::TransactionSplit(err.to_string())),
                    }
                }
                _ => {
                    Err(RequestNotSuccessful::new(response.status(), response.text().await?).into())
                }
            },
            Err(err) => Err(err),
        }
    }

    /// Update a transaction split details on your integration.
    ///
    /// Takes in the following parameters:
    ///     - `split_id`: Id of the transaction split.
    ///     - `split_name`: updated name for the split.
    ///     - `split_status`: updated states for the split.
    ///     - `bearer_type`: (Optional) updated bearer type for the split.
    ///     - `bearer_subaccount`: (Optional) updated bearer subaccount for the split
    pub async fn update_transaction_split(
        &self,
        split_id: String,
        body: TransactionSplit,
    ) -> PaystackResult<TransactionSplitResponse> {
        let url = format!("{}/split/{}", BASE_URL, split_id);

        match self.put_request(&url, body).await {
            Ok(respone) => match respone.status() {
                reqwest::StatusCode::OK => match respone.json::<TransactionSplitResponse>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(PaystackError::TransactionSplit(err.to_string())),
                },
                _ => Err(RequestNotSuccessful::new(respone.status(), respone.text().await?).into()),
            },
            Err(err) => Err(err),
        }
    }

    /// Add a Subaccount to a Transaction Split, or update the share of an existing Subaccount in a Transaction Split
    ///
    /// Takes in the following parameters:
    ///     - `split_id`: Id of the transaction split to update.
    ///     - `body`: Subacount to add to the transaction split.
    pub async fn add_or_update_subaccount_split(
        &self,
        split_id: String,
        body: Subaccount,
    ) -> PaystackResult<TransactionSplitResponse> {
        let url = format!("{}/split/{}/subaccount/add", BASE_URL, split_id);

        match self.post_request(&url, body).await {
            Ok(response) => match response.status() {
                reqwest::StatusCode::OK => {
                    match response.json::<TransactionSplitResponse>().await {
                        Ok(content) => Ok(content),
                        Err(err) => Err(PaystackError::TransactionSplit(err.to_string())),
                    }
                }
                _ => {
                    Err(RequestNotSuccessful::new(response.status(), response.text().await?).into())
                }
            },
            Err(err) => Err(err),
        }
    }

    /// Remove a subaccount from a transaction split.
    ///
    /// Takes in the following parameters
    ///     - split_id: Id of the transaction split
    ///     - subaccount: subaccount code to remove
    pub async fn remove_subaccount_from_transaction_split(
        &self,
        split_id: String,
        subaccount: String,
    ) -> PaystackResult<ResponseWithoutData> {
        let url = format!("{}/split/{}/subaccount/remove", BASE_URL, split_id);

        match self.post_request(&url, subaccount).await {
            Ok(response) => match response.status() {
                reqwest::StatusCode::OK => match response.json::<ResponseWithoutData>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(PaystackError::TransactionSplit(err.to_string())),
                },
                _ => {
                    Err(RequestNotSuccessful::new(response.status(), response.text().await?).into())
                }
            },
            Err(err) => Err(err),
        }
    }
}
