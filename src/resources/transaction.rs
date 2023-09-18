//! Transactions
//! =============
//! TThe Transactions API allows you create and manage payments on your integration.

use crate::{
    get_request, post_request, Channel, ChargeBody, Currency, Error, ExportTransactionResponse,
    PaystackResult, Status, TransactionResponse, TransactionStatus, TransactionStatusList,
    TransactionTimeline, TransactionTotalsResponse,
};
use derive_builder::Builder;
use reqwest::StatusCode;
use serde::Serialize;

/// This struct is used to create a transaction body for creating a transaction using the Paystack API.
/// This struct should be created using the `InitializeTransactionBodyBuilder`
/// The Builder derivation allows for the automatic implementation of the builder pattern.
#[derive(Serialize, Debug, Default, Builder)]
pub struct InitializeTransactionBody {
    /// Amount should be in the smallest unit of the currency e.g. kobo if in NGN and cents if in USD
    amount: String,
    /// Customer's email address
    email: String,
    /// Currency in which amount should be charged (NGN, GHS, ZAR or USD). Defaults to your integration currency.
    #[builder(default = "None")]
    currency: Option<Currency>,
    /// Unique transaction reference. Only -, ., = and alphanumeric characters allowed.
    #[builder(default = "None")]
    reference: Option<String>,
    /// Fully qualified url, e.g. https://example.com/ . Use this to override the callback url provided on the dashboard for this transaction
    #[builder(default = "None")]
    callback_url: Option<String>,
    /// If transaction is to create a subscription to a predefined plan, provide plan code here. This would invalidate the value provided in `amount`
    #[builder(default = "None")]
    plan: Option<String>,
    /// Number of times to charge customer during subscription to plan
    #[builder(default = "None")]
    invoice_limit: Option<u32>,
    /// Stringified JSON object of custom data. Kindly check the `Metadata` struct for more information.
    #[builder(default = "None")]
    metadata: Option<String>,
    /// An array of payment channels to control what channels you want to make available to the user to make a payment with.
    /// Available channels include: `["card", "bank", "ussd", "qr", "mobile_money", "bank_transfer", "eft"]`
    #[builder(default = "None")]
    channels: Option<Vec<Channel>>,
    /// The split code of the transaction split. e.g. `SPL_98WF13Eb3w`
    #[builder(default = "None")]
    split_code: Option<String>,
    /// The code for the subaccount that owns the payment. e.g. `ACCT_8f4s1eq7ml6rlzj`
    #[builder(default = "None")]
    subaccount: Option<String>,
    /// An amount used to override the split configuration for a single split payment.
    /// If set, the amount specified goes to the main account regardless of the split configuration.
    #[builder(default = "None")]
    transaction_charge: Option<u32>,
    /// Who bears Paystack charges? `account` or `subaccount` (defaults to account).
    #[builder(default = "None")]
    bearer: Option<String>,
}

/// This struct is used to create a partial debit transaction body for creating a partial debit using the Paystack API.
/// This struct should be created using the `PartialDebitTransactionBodyBuilder`
/// The derive Builder allows for the automatic creation of the BuilderPattern
#[derive(Debug, Clone, Serialize, Default, Builder)]
pub struct PartialDebitTransactionBody {
    /// Authorization Code
    authorization_code: String,
    /// Specify the currency you want to debit. Allowed values are NGN or GHS.
    currency: Currency,
    /// Amount should be in the subunit of the supported currency
    amount: String,
    /// Customer's email address (attached to the authorization code)
    email: String,
    /// Unique transaction reference. Only `-`, `.`, `=` and alphanumeric characters allowed.
    #[builder(default = "None")]
    reference: Option<String>,
    /// Minimum amount to charge
    #[builder(default = "None")]
    at_least: Option<String>,
}

/// A Struct to hold all the functions of the transaction API route
#[derive(Debug, Clone)]
pub struct Transaction {
    /// Paystack API Key
    api_key: String,
}

static BASE_URL: &str = "https://api.paystack.co";

impl Transaction {
    /// Constructor for the transaction object
    pub fn new(key: String) -> Self {
        Transaction { api_key: key }
    }

    /// Initialize a transaction from your backend.
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
        reference: &String,
    ) -> PaystackResult<TransactionStatus> {
        let url = format!("{}/transaction/verify/{}", BASE_URL, reference);

        match get_request(&self.api_key, &url, None).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<TransactionStatus>().await {
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
    ) -> PaystackResult<TransactionStatus> {
        let url = format!("{}/transaction/{}", BASE_URL, transaction_id);

        match get_request(&self.api_key, &url, None).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<TransactionStatus>().await {
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
        charge: ChargeBody,
    ) -> PaystackResult<TransactionStatus> {
        let url = format!("{}/transaction/charge_authorization", BASE_URL);

        match post_request(&self.api_key, &url, charge).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<TransactionStatus>().await {
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

        match get_request(&self.api_key, &url, None).await {
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

        let query = vec![
            ("status", status.unwrap_or(Status::Success).to_string()),
            ("currency", currency.unwrap_or(Currency::EMPTY).to_string()),
            ("settled", settled),
        ];

        match get_request(&self.api_key, &url, Some(query)).await {
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
        transaction_body: PartialDebitTransactionBody,
    ) -> PaystackResult<TransactionStatus> {
        let url = format!("{}/transaction/partial_debit", BASE_URL);

        match post_request(&self.api_key, &url, transaction_body).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<TransactionStatus>().await {
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
