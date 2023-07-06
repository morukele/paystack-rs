//! Client
//! ===========
//! The file for the Paystack API client and it's associated functions

extern crate reqwest;
extern crate serde_json;

use crate::{
    Charge, PaystackResult, RequestNotSuccessful, Transaction, TransactionResponse,
    TransactionStatus, TransactionStatusList, TransactionTimeline,
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

    /// This method initalizes a new transaction using the Paystack API.
    ///
    /// It takes a Transaction type as its parameter
    pub async fn initialize_transaction(
        &self,
        transaction_body: Transaction,
    ) -> PaystackResult<TransactionResponse> {
        let url = format!("{}/transaction/initialize", BASE_URL);

        let response = self
            .client
            .post(url)
            .bearer_auth(&self.api_key)
            .header("Content-Type", "application/json")
            .json(&transaction_body)
            .send()
            .await?;

        if response.error_for_status_ref().is_err() {
            return Err(
                RequestNotSuccessful::new(response.status(), response.text().await?).into(),
            );
        }

        let content = response.json::<TransactionResponse>().await?;

        Ok(content)
    }

    /// This method confirms the status of a transaction.
    ///
    /// It takes the following parameters:
    ///     - reference: The transaction reference used to intiate the transaction
    pub async fn verify_transaction(&self, reference: String) -> PaystackResult<TransactionStatus> {
        let url = format!("{}/transaction/verify/{}", BASE_URL, reference);

        let response = self
            .client
            .get(url)
            .bearer_auth(&self.api_key)
            .header("Content-Type", "application/json")
            .send()
            .await?;

        if response.error_for_status_ref().is_err() {
            return Err(
                RequestNotSuccessful::new(response.status(), response.text().await?).into(),
            );
        }

        let content = response.json::<TransactionStatus>().await?;
        Ok(content)
    }

    /// This method returns a Vec of transactions carried out on your integrations.
    ///
    /// The method takes the following parameters:
    ///     - perPage (Optional): Number of transactions to return. If None is passed as the parameter, the last 10 transactions are returned.
    ///     - status (Optional): Filter transactions by status (`failed`, `success`, `abandoned`).
    ///
    pub async fn list_transactions(
        &self,
        number_of_transactions: Option<u32>,
        status: Option<String>,
    ) -> PaystackResult<TransactionStatusList> {
        let url = format!("{}/transaction", BASE_URL);
        let query = vec![
            ("perPage", number_of_transactions.unwrap_or(10).to_string()),
            ("status", status.unwrap()),
        ];

        let response = self
            .client
            .get(url)
            .query(&query)
            .bearer_auth(&self.api_key)
            .header("Content-Type", "application.json")
            .send()
            .await?;

        if response.error_for_status_ref().is_err() {
            return Err(
                RequestNotSuccessful::new(response.status(), response.text().await?).into(),
            );
        }

        let contents = response.json::<TransactionStatusList>().await?;
        Ok(contents)
    }

    /// Get details of a transaction carried out on your integration
    ///
    /// This methods take the Id of the desired transaction as a parameter
    pub async fn fetch_transactions(
        &self,
        transaction_id: u32,
    ) -> PaystackResult<TransactionStatus> {
        let url = format!("{}/transaction/{}", BASE_URL, transaction_id);

        let response = self
            .client
            .get(url)
            .bearer_auth(&self.api_key)
            .header("Content-Type", "application/json")
            .send()
            .await?;

        if response.error_for_status_ref().is_err() {
            return Err(
                RequestNotSuccessful::new(response.status(), response.text().await?).into(),
            );
        }

        let content = response.json::<TransactionStatus>().await?;

        Ok(content)
    }

    /// All authorizations marked as reusable can be charged with this endpoint whenever you need to receive payments
    ///
    /// This function takes a Charge Struct as parameter
    pub async fn charge_authorization(&self, charge: Charge) -> PaystackResult<TransactionStatus> {
        let url = format!("{}/transaction/charge_authorization", BASE_URL);

        let response = self
            .client
            .post(url)
            .bearer_auth(&self.api_key)
            .header("Content-Type", "application/json")
            .json(&charge)
            .send()
            .await?;

        if response.error_for_status_ref().is_err() {
            return Err(
                RequestNotSuccessful::new(response.status(), response.text().await?).into(),
            );
        }
        let content = response.json::<TransactionStatus>().await?;

        Ok(content)
    }

    /// View the timeline of a transaction
    ///
    /// This function takes in the Transaction id or reference as a parameter
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
                return Err(crate::PaystackError::Transaction(
                    "Transaction Id or Reference is need to view transaction timeline".to_string(),
                ))
            }
        };

        let url = url.unwrap(); // Send the error back up the function

        let response = self
            .client
            .get(url)
            .bearer_auth(&self.api_key)
            .header("Content-Type", "application/json")
            .send()
            .await?;

        if response.error_for_status_ref().is_err() {
            return Err(
                RequestNotSuccessful::new(response.status(), response.text().await?).into(),
            );
        }
        let content = response.json::<TransactionTimeline>().await?;

        Ok(content)
    }
}
