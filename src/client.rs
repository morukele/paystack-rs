//! Client
//! ===========
//! The file for the Paystack API client and it's associated functions

extern crate reqwest;
extern crate serde_json;

use reqwest::StatusCode;
use std::fmt::Debug;
use crate::{ChargeBody, Currency, ExportTransactionResponse, PartialDebitTransactionBody, Error, PaystackResult, ResponseWithoutData, Status, Subaccount, InitializeTransactionBody, TransactionResponse, CreateTransactionSplitBody, TransactionSplitListResponse, TransactionSplitResponse, TransactionStatus, TransactionStatusList, TransactionTimeline, TransactionTotalsResponse, put_request, UpdateTransactionSplitBody, CreateSubaccountBody, CreateSubAccountResponse};
use crate::{get_request, post_request};

static BASE_URL: &str = "https://api.paystack.co";

/// This is the struct that allows you to authenticate to the PayStack API.
/// It contains the API key which allows you to interact with the API.
#[derive(Clone, Debug)]
pub struct PaystackClient {
    api_key: String,
}

impl PaystackClient {
    /// This method creates a new PayStack client with the specified API key.
    ///
    /// It takes the following parameters:
    ///     - key: Paystack API key.
    pub fn new(key: String) -> Self {
        Self {
            api_key: key,
        }
    }

    /// This method initializes a new transaction using the Paystack API.
    ///
    /// It takes a Transaction type as its parameter
    pub async fn initialize_transaction(
        &self,
        transaction_body: InitializeTransactionBody,
    ) -> PaystackResult<TransactionResponse> {
        let url = format!("{}/transaction/initialize", BASE_URL);

        match post_request(&self.api_key, &url, transaction_body).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<TransactionResponse>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(Error::Transaction(err.to_string())),
                },
                _ => {
                    Err(Error::RequestNotSuccessful(response.status().to_string(), response.text().await?))
                }
            },
            Err(err) => Err(Error::FailedRequest(err.to_string())),
        }
    }

    /// This method confirms the status of a transaction.
    ///
    /// It takes the following parameters:
    ///     - reference: The transaction reference used to initiate the transaction
    pub async fn verify_transaction(
        &self,
        reference: &String,
    ) -> PaystackResult<TransactionStatus> {
        let url = format!("{}/transaction/verify/{}", BASE_URL, reference);

        match get_request(&self.api_key, &url, None).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<TransactionStatus>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(Error::Transaction(err.to_string())),
                },
                _ => {
                    Err(Error::RequestNotSuccessful(response.status().to_string(), response.text().await?))
                }
            },
            Err(err) => Err(Error::FailedRequest(err.to_string())),
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

        match get_request(&self.api_key, &url, Some(query)).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<TransactionStatusList>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(Error::Transaction(err.to_string())),
                },
                _ => {
                    Err(Error::RequestNotSuccessful(response.status().to_string(), response.text().await?))
                }
            },
            Err(err) => Err(Error::FailedRequest(err.to_string())),
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

        match get_request(&self.api_key, &url, None).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<TransactionStatus>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(Error::Transaction(err.to_string())),
                },
                _ => {
                    Err(Error::RequestNotSuccessful(response.status().to_string(), response.text().await?))
                }
            },
            Err(err) => Err(Error::FailedRequest(err.to_string())),
        }
    }

    /// All authorizations marked as reusable can be charged with this endpoint whenever you need to receive payments
    ///
    /// This function takes a Charge Struct as parameter
    pub async fn charge_authorization(&self, charge: ChargeBody) -> PaystackResult<TransactionStatus> {
        let url = format!("{}/transaction/charge_authorization", BASE_URL);

        match post_request(&self.api_key,&url, charge).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<TransactionStatus>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(Error::Charge(err.to_string())),
                },
                _ => {
                    Err(Error::RequestNotSuccessful(response.status().to_string(), response.text().await?))
                }
            },
            Err(err) => Err(Error::FailedRequest(err.to_string())),
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

        match get_request(&self.api_key, &url, None).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<TransactionTimeline>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(Error::Transaction(err.to_string())),
                },
                _ => {
                    Err(Error::RequestNotSuccessful(response.status().to_string(), response.text().await?))
                }
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

        match get_request(&self.api_key, &url, None).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<TransactionTotalsResponse>().await
                {
                    Ok(content) => Ok(content),
                    Err(err) => Err(Error::Transaction(err.to_string())),
                },
                _ => {
                    Err(Error::RequestNotSuccessful(response.status().to_string(), response.text().await?))
                }
            },
            Err(err) => Err(Error::FailedRequest(err.to_string())),
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

        match get_request(&self.api_key, &url, Some(query)).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<ExportTransactionResponse>().await
                {
                    Ok(content) => Ok(content),
                    Err(err) => Err(Error::Transaction(err.to_string())),
                },
                _ => {
                    Err(Error::RequestNotSuccessful(response.status().to_string(), response.text().await?))
                }
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
        transaction_body: PartialDebitTransactionBody,
    ) -> PaystackResult<TransactionStatus> {
        let url = format!("{}/transaction/partial_debit", BASE_URL);

        match post_request(&self.api_key, &url, transaction_body).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<TransactionStatus>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(Error::Transaction(err.to_string())),
                },
                _ => {
                    Err(Error::RequestNotSuccessful(response.status().to_string(), response.text().await?))
                }
            },
            Err(err) => Err(Error::FailedRequest(err.to_string())),
        }
    }

    /// Create a split payment on your integration.
    ///
    /// This method takes a TransactionSplit object as a parameter.
    pub async fn create_transaction_split(
        &self,
        split_body: CreateTransactionSplitBody,
    ) -> PaystackResult<TransactionSplitResponse> {
        let url = format!("{}/split", BASE_URL);

        match post_request(&self.api_key, &url, split_body).await {
            Ok(response) => match response.status() {
                StatusCode::OK => {
                    match response.json::<TransactionSplitResponse>().await {
                        Ok(content) => Ok(content),
                        Err(err) => Err(Error::TransactionSplit(err.to_string())),
                    }
                }
                _ => {
                    Err(Error::RequestNotSuccessful(response.status().to_string(), response.text().await?))
                }
            },
            Err(err) => Err(Error::FailedRequest(err.to_string())),
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

        match get_request(&self.api_key, &url, Some(query)).await {
            Ok(response) => match response.status() {
                StatusCode::OK => {
                    match response.json::<TransactionSplitListResponse>().await {
                        Ok(content) => Ok(content),
                        Err(err) => Err(Error::TransactionSplit(err.to_string())),
                    }
                }
                _ => {
                    Err(Error::RequestNotSuccessful(response.status().to_string(), response.text().await?))
                }
            },
            Err(err) => Err(Error::FailedRequest(err.to_string())),
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

        match get_request(&self.api_key, &url, None).await {
            Ok(response) => match response.status() {
                StatusCode::OK => {
                    match response.json::<TransactionSplitResponse>().await {
                        Ok(content) => Ok(content),
                        Err(err) => Err(Error::TransactionSplit(err.to_string())),
                    }
                }
                _ => {
                    Err(Error::RequestNotSuccessful(response.status().to_string(), response.text().await?))
                }
            },
            Err(err) => Err(Error::FailedRequest(err.to_string())),
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
        body: UpdateTransactionSplitBody,
    ) -> PaystackResult<TransactionSplitResponse> {
        let url = format!("{}/split/{}", BASE_URL, split_id);

        match put_request(&self.api_key, &url, body).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<TransactionSplitResponse>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(Error::TransactionSplit(err.to_string())),
                },
                _ => Err(Error::RequestNotSuccessful(response.status().to_string(), response.text().await?)),
            },
            Err(err) => Err(Error::FailedRequest(err.to_string())),
        }
    }

    /// Add a Subaccount to a Transaction Split, or update the share of an existing Subaccount in a Transaction Split
    ///
    /// Takes in the following parameters:
    ///     - `split_id`: Id of the transaction split to update.
    ///     - `body`: Subaccount to add to the transaction split.
    pub async fn add_or_update_subaccount_split(
        &self,
        split_id: String,
        body: Subaccount,
    ) -> PaystackResult<TransactionSplitResponse> {
        let url = format!("{}/split/{}/subaccount/add", BASE_URL, split_id);

        match post_request(&self.api_key, &url, body).await {
            Ok(response) => match response.status() {
                StatusCode::OK => {
                    match response.json::<TransactionSplitResponse>().await {
                        Ok(content) => Ok(content),
                        Err(err) => Err(Error::TransactionSplit(err.to_string())),
                    }
                }
                _ => {
                    Err(Error::RequestNotSuccessful(response.status().to_string(), response.text().await?))
                }
            },
            Err(err) => Err(Error::FailedRequest(err.to_string())),
        }
    }

    /// Remove a subaccount from a transaction split.
    ///
    /// Takes in the following parameters
    ///     - split_id: Id of the transaction split
    ///     - subaccount: subaccount code to remove
    pub async fn remove_subaccount_from_transaction_split(
        &self,
        split_id: &String,
        subaccount: &String,
    ) -> PaystackResult<ResponseWithoutData> {
        let url = format!("{}/split/{}/subaccount/remove", BASE_URL, split_id);

        match post_request(&self.api_key, &url, subaccount).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<ResponseWithoutData>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(Error::TransactionSplit(err.to_string())),
                },
                _ => {
                    Err(Error::RequestNotSuccessful(response.status().to_string(), response.text().await?))
                }
            },
            Err(err) => Err(Error::FailedRequest(err.to_string())),
        }
    }


    /// Creates a new subaccount.
    ///
    /// Takes in the following parameters
    ///     - body: subaccount to create.
    pub async fn create_subaccount(
        &self,
        body: CreateSubaccountBody
    ) -> PaystackResult<CreateSubAccountResponse> {
        let url = format!("{}/subaccount", BASE_URL);

        match post_request(&self.api_key, &url, body).await {
            Ok(response) => match response.status() {
                StatusCode::CREATED => match response.json::<CreateSubAccountResponse>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(Error::Subaccount(err.to_string()))
                },
                _ => {
                  Err(Error::RequestNotSuccessful(response.status().to_string(), response.text().await?))
                }
            },
            Err(err) => Err(Error::FailedRequest(err.to_string()))
        }
    }
}
