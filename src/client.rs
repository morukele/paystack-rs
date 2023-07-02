extern crate reqwest;
extern crate serde_json;

use crate::error::{PaystackError, RequestNotSuccessful};
use crate::response::TransactionStatus;
use crate::{PaystackResult, TransactionResponse};

static BASE_URL: &str = "https://api.paystack.co";

/// This is the struct that allows you to authenticate to the PayStack API.
/// It contains the API key which allows you to interact with the API.
#[derive(Clone, Debug)]
pub struct PaystackClient {
    client: reqwest::Client,
    api_key: String,
}

/// This struct is used to create a transaction body for creating a transaction using the Paystack API.
///
/// The struct has the following fields:
///     - amount: Amount should be in the smallest unit of the currency e.g. kobo if in NGN and cents if in USD
///     - email: Customer's email address
///     - currency (Optional): The transaction currency (NGN, GHS, ZAR or USD). Defaults to your integration currency.
///     - plan (Optional): If transaction is to create a subscription to a predefined plan, provide plan code here.
///       This would invalidate the value provided in amount
///     - transaction_charge (Optional): An amount used to override the split configuration for a single split payment.
///       If set, the amount specified goes to the main account regardless of the split configuration.
///     - bearer (Optional): Who bears Paystack charges? account or subaccount (defaults to account).
#[derive(serde::Serialize)]
pub struct TransactionBody {
    pub amount: String,
    pub email: String,
    pub currency: Option<String>,
    pub plan: Option<String>,
    pub transaction_charge: Option<i32>,
    pub bearer: Option<String>,
}

impl PaystackClient {
    /// Create a new PayStack client with the specified API key.
    ///
    /// It takes the following parameters:
    ///     - key: Paystack API key.
    pub fn new<S: Into<String>>(key: S) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key: key.into(),
        }
    }

    /// Initalize a new transaction using the Paystack API.
    ///
    /// The function takes a TransactionBody type as its parameter
    ///
    pub async fn initialize_transaction(
        &self,
        transaction_body: TransactionBody,
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

        if let Err(_ex) = response.error_for_status_ref() {
            return Err(
                RequestNotSuccessful::new(response.status(), response.text().await?).into(),
            );
        }

        let contents = response.json::<TransactionResponse>().await?;

        Ok(contents)
    }

    /// This function confirms the status of a transaction.
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

        if let Err(ex) = response.error_for_status_ref() {
            return Err(PaystackError::Generic(ex.to_string()));
        }

        let contents = response.json::<TransactionStatus>().await?;
        Ok(contents)
    }
}
