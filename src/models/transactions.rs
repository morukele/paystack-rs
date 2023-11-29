//! Transactions Models
//! ====================
//! This file contains the models for working with the transactions endpoint.
use crate::{Authorization, Channel, Currency, Customer, MetaData, SubaccountData};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// This struct is used to create a transaction body for creating a transaction using the Paystack API.
/// This struct should be created using the `InitializeTransactionBodyBuilder`.
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

/// This struct represents the response of the Paystack transaction initialization.
#[derive(Deserialize, Debug, Clone)]
pub struct TransactionResponse {
    /// This lets you know if your request was successful or not.
    pub status: bool,
    /// This is a summary of the response and its status.
    pub message: String,
    /// This contains the results of your request.
    pub data: TransactionResponseData,
}

/// This struct represents the data of the transaction response.
#[derive(Deserialize, Debug, Clone)]
pub struct TransactionResponseData {
    /// Generated URL to authorize the transaction.
    pub authorization_url: String,
    /// Access code of the transaction.
    pub access_code: String,
    /// Reference of the transaction.
    pub reference: String,
}

/// This struct represents the transaction status response.
#[derive(Deserialize, Debug, Clone)]
pub struct TransactionStatusResponse {
    /// This lets you know if your request was successful or not.
    pub status: bool,
    /// This is a summary of the response and its status.
    pub message: String,
    /// This contains the results of your request.
    /// In this case, it is a single object.
    pub data: TransactionStatusData,
}

/// This struct represents the data of the transaction status response.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TransactionStatusData {
    /// Id of the Transaction
    pub id: Option<u32>,
    /// Status of the Transaction. It can be `success`, `abandoned` or `failed`
    pub status: Option<String>,
    /// Reference of the Transaction
    pub reference: Option<String>,
    /// Amount of the transaction in the lowest denomination of the currency e.g. Kobo for NGN and cent for USD.
    pub amount: Option<u32>,
    /// Message from the transaction.
    pub message: Option<String>,
    /// Response from the payment gateway.
    pub gateway_response: Option<String>,
    /// Time the Transaction was completed.
    pub paid_at: Option<String>,
    /// Time the Transaction was created.
    pub created_at: Option<String>,
    /// Transaction channel. It can be `card` or `bank`.
    pub channel: Option<String>,
    /// Currency code of the Transaction e.g. `NGN for Nigerian Naira` and `USD for US Dollar`.
    pub currency: Option<String>,
    /// IP address of the computers the Transaction has passed through.
    pub ip_address: Option<String>,
    /// Meta data associated with the Transaction.
    pub metadata: Option<String>,
    /// Transaction fees to override the default fees specified in the integration.
    pub fees: Option<i32>,
    /// Transaction customer data.
    pub customer: Option<Customer>,
    /// Transaction authorization data.
    pub authorization: Option<Authorization>,
}

/// Represents the response of the total amount received on your account
#[derive(Debug, Deserialize, Serialize)]
pub struct TransactionTotalsResponse {
    /// This lets you know if your request was successful or not.
    pub status: bool,
    /// This is a summary of the response and its status.
    pub message: String,
    /// This contains the results of your request.
    pub data: TransactionTotalData,
}

/// Transaction total data.
#[derive(Debug, Deserialize, Serialize)]
pub struct TransactionTotalData {
    /// Total number of transactions in the integration.
    pub total_transactions: Option<u32>,
    /// Total of unique number of customers in the integration.
    pub unique_customers: Option<u32>,
    /// Total volume of transaction in the integration.
    pub total_volume: Option<u32>,
    /// Total volume of transaction broken down by currency.
    pub total_volume_by_currency: Option<Vec<VolumeByCurrency>>,
    /// Total volume of pending transfers.
    pub pending_transfers: Option<u32>,
    /// Total volume of pending transfer broken down by currency.
    pub pending_transfers_by_currency: Option<Vec<VolumeByCurrency>>,
}

/// Transaction volume by currency.
#[derive(Debug, Deserialize, Serialize)]
pub struct VolumeByCurrency {
    /// Currency code.
    pub currency: String,
    /// Amount in the lowest denomination of the currency.
    pub amount: u32,
}

/// Represents the response of the export transaction.
#[derive(Debug, Serialize, Deserialize)]
pub struct ExportTransactionResponse {
    /// This lets you know if your request was successful or not.
    pub status: bool,
    /// This is a summary of the response and its status.
    pub message: String,
    /// This contains the results of your request.
    pub data: ExportTransactionData,
}

/// Export transaction response data.
#[derive(Debug, Serialize, Deserialize)]
pub struct ExportTransactionData {
    /// Path to download the exported transaction file.
    pub path: String,
}

/// Represents the response of the partial debit transaction.
#[derive(Debug, Deserialize, Serialize)]
pub struct PartialDebitTransactionResponse {
    /// This lets you know if your request was successful or not.
    pub status: bool,
    /// This is a summary of the response and its status.
    pub message: String,
    /// This contains the results of your request.
    pub data: TransactionStatusData,
}

/// Represents the JSON response containing percentage split information.
#[derive(Debug, Deserialize, Serialize)]
pub struct TransactionSplitResponse {
    /// The status of the JSON response.
    pub status: bool,
    /// The message associated with the JSON response.
    pub message: String,
    /// The percentage split data.
    pub data: SplitData,
}

/// Represents the percentage split data received in the JSON response.
#[derive(Debug, Deserialize, Serialize)]
pub struct SplitData {
    /// The ID of the percentage split.
    pub id: u32,
    /// The name of the percentage split.
    pub name: String,
    /// The type of the percentage split.
    #[serde(rename = "type")]
    pub split_type: String,
    /// The currency used for the percentage split.
    pub currency: String,
    /// The integration associated with the percentage split.
    pub integration: u32,
    /// The domain associated with the percentage split.
    pub domain: String,
    /// The split code of the percentage split.
    pub split_code: String,
    /// Indicates whether the percentage split is active or not.
    pub active: Option<bool>,
    /// The bearer type of the percentage split.
    pub bearer_type: String,
    /// The subaccount ID of the bearer associated with the percentage split.
    pub bearer_subaccount: u32,
    /// The creation timestamp of the percentage split.
    pub created_at: Option<String>,
    /// The last update timestamp of the percentage split.
    pub updated_at: Option<String>,
    /// The list of sub accounts involved in the percentage split.
    pub subaccounts: Vec<SubaccountData>,
    /// The total count of subaccounts in the percentage split.
    pub total_subaccounts: u32,
}

/// Represents the JSON response containing percentage split information.
#[derive(Debug, Deserialize, Serialize)]
pub struct TransactionSplitListResponse {
    /// The status of the JSON response.
    pub status: bool,
    /// The message associated with the JSON response.
    pub message: String,
    /// The percentage split data.
    pub data: Vec<SplitData>,
}

/// This struct represents a list of transaction status.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TransactionStatusListResponse {
    /// This lets you know if your request was successful or not.
    pub status: bool,
    /// This is a summary of the response and its status.
    pub message: String,
    /// This contains the results of your request.
    /// In this case, it is a vector of objects.
    pub data: Vec<TransactionStatusData>,
    /// The meta key is used to provide context for the contents of the data key.
    pub meta: MetaData,
}

/// This struct represents the transaction timeline.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TransactionTimelineResponse {
    /// This lets you know if your request was successful or not.
    pub status: bool,
    /// This is a summary of the response and its status.
    pub message: String,
    /// This contains the results of your request.
    pub data: TransactionTimelineData,
}

/// This struct represents the transaction timeline data.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TransactionTimelineData {
    /// Time spent in carrying out the transaction in ms.
    pub time_spent: Option<u32>,
    /// Number of attempts for the transaction.
    pub attempts: Option<u32>,
    /// Authentication use for the transaction.
    pub authentication: Option<String>,
    /// Number of errors for the transaction.
    pub errors: Option<u32>,
    /// Success status of the transaction.
    pub success: Option<bool>,
    /// If transaction was carried out with mobile.
    pub mobile: Option<bool>,
    /// Transaction inputs i.e. messages associated with the transaction.
    pub input: Option<String>,
    /// Transaction channel.
    pub channel: Option<String>,
    /// Transaction history.
    pub history: Option<Vec<TransactionHistoryResponse>>,
}

/// This struct represents the transaction history data
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TransactionHistoryResponse {
    /// Transaction action.
    #[serde(rename = "type")]
    pub action_type: String,
    /// Description of the action.
    pub message: String,
    /// Time action was taken in ms.
    pub time: u32,
}
