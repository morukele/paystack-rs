//! Response
//! ==========
//! This file contains the structs needed to represent the different response
//! of the Paystack API.

use serde::{Deserialize, Serialize};

/// This struct represents the response of the Paystack transaction initalization.
#[derive(Deserialize, Debug, Clone)]
pub struct TransactionResponse {
    /// This lets you know if your request was succesful or not.
    pub status: bool,
    /// This is a summary of the response and its status.
    pub message: String,
    /// This contains the results of your request.
    pub data: TransactionResponseData,
}

/// This struct represents the data of the transaction response.
#[derive(Deserialize, Debug, Clone)]
pub struct TransactionResponseData {
    /// Generated URL to authroize the transaction.
    pub authorization_url: String,
    /// Access code of the transaction.
    pub access_code: String,
    /// Reference of the transaction.
    pub reference: String,
}

/// This struct represents the transaction status response.
#[derive(Deserialize, Debug, Clone)]
pub struct TransactionStatus {
    /// This lets you know if your request was succesful or not.
    pub status: bool,
    /// This is a summary of the response and its status.
    pub message: String,
    /// This contains the results of your request.
    /// In this case, it is a single object.
    pub data: TransactionStatusData,
}

/// This struct represents a list of transaction status.
#[derive(Deserialize, Debug, Clone)]
pub struct TransactionStatusList {
    /// This lets you know if your request was succesful or not.
    pub status: bool,
    /// This is a summary of the response and its status.
    pub message: String,
    /// This contains the results of your request.
    /// In this case, it is a vector of objects.
    pub data: Vec<TransactionStatusData>,
}

/// This struct represents the transaction timeline.
#[derive(Deserialize, Debug, Clone)]
pub struct TransactionTimeline {
    /// This lets you know if your request was successful or not.
    pub status: bool,
    /// This is a summary of the response and its status.
    pub message: String,
    /// This contains the results of your request.
    pub data: TransactionTimelineData,
}

/// This struct represents the transaction timeline data.
#[derive(Deserialize, Debug, Clone)]
pub struct TransactionTimelineData {
    /// Time spent in carrying out the transaction in ms.
    pub time_spent: Option<u32>,
    /// Number of attempts for the transaction.
    pub attempts: Option<u32>,
    /// Authentication use for the transaction.
    pub authentication: Option<String>,
    /// Number of errors for the transaction.
    pub errors: Option<u32>,
    /// Sucess status of the transaction.
    pub success: Option<bool>,
    /// If transaction was carried out with mobile.
    pub mobile: Option<bool>,
    /// Transaction inputs i.e. messages associated with the transaction.
    pub input: Option<String>,
    /// Transaction channel.
    pub channel: Option<String>,
    /// Trasnaction history.
    pub history: Option<Vec<TranasctionHistory>>,
}

/// This struct represents the transaction history data
#[derive(Deserialize, Debug, Clone)]
pub struct TranasctionHistory {
    /// Transaction action.
    #[serde(rename = "type")]
    pub action_type: String,
    /// Description of the action.
    pub message: String,
    /// Time action was taken in ms.
    pub time: u32,
}

/// This struct represents the data of the transaction status response.
#[derive(Deserialize, Debug, Clone)]
pub struct TransactionStatusData {
    /// Id of the Transaction
    pub id: Option<u32>,
    /// Status of the Transaction. It can be `success`, `abadoned` or `failed`
    pub status: Option<String>,
    /// Reference of the Transaction
    pub reference: Option<String>,
    /// Amount of the transaction in the lowest denomination of the currency e.g. Kobo for NGN and cent for USD.
    pub amount: Option<u32>,
    /// Message from the transaction.
    pub message: Option<String>,
    /// Response from the paymeent gateway.
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
    /// Transaction fees to overide the default fees specified in the integration.
    pub fees: Option<i32>,
    /// Transaction customer data.
    pub customer: Option<Customer>,
    /// Transaction authorization data.
    pub authorization: Option<Authorization>,
}

/// This struct represents the authorization data of the transaction status response
#[derive(Debug, Deserialize, Clone)]
pub struct Authorization {
    /// Authorization code generated for the Transaction.
    pub authorization_code: Option<String>,
    /// Bin number for Transaction authorization.
    pub bin: Option<String>,
    /// Last 4 digits of authorized card.
    pub last4: Option<String>,
    /// Authorized card expiry month.
    pub exp_month: Option<String>,
    /// Authorized card expirey year.
    pub exp_year: Option<String>,
    /// Authorization channel. It could be `card` or `bank`.
    pub channel: Option<String>,
    /// Type of card used in the Authorization
    pub card_type: Option<String>,
    /// Name of bank associated with the Authorization.
    pub bank: Option<String>,
    /// Country code of the Authroization.
    pub country_code: Option<String>,
    /// Brand of of the Authroization if it is a card.
    pub brand: Option<String>,
    /// Specifies if the Authroization is reuseable.
    pub reusable: Option<bool>,
    /// Signature of the Authorization.
    pub signature: Option<String>,
    /// Name of the account associated with the authorization.
    pub account_name: Option<String>,
}

/// This struct represents the Paystack customer data
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Customer {
    /// Customer's Id.
    pub id: Option<u32>,
    /// Customer's first name.
    pub first_name: Option<String>,
    /// Customer's last name.
    pub last_name: Option<String>,
    /// Customer's email address.
    pub email: Option<String>,
    /// Customer's code.
    pub customer_code: String,
    /// Customer's phone number.
    pub phone: Option<String>,
    /// Customer's metadata.
    pub metadata: Option<String>,
    /// Customer's risk action.
    pub risk_action: Option<String>,
    /// Customer's phone number in international format.
    pub international_format_phone: Option<String>,
}

/// Represents the response of the total amount received on your account
#[derive(Debug, Deserialize)]
pub struct TransactionTotalsResponse {
    /// This lets you know if your request was succesful or not.
    pub status: bool,
    /// This is a summary of the response and its status.
    pub message: String,
    /// This contains the results of your request.
    pub data: TransactionTotalData,
}

/// Transaction total data.
#[derive(Debug, Deserialize)]
pub struct TransactionTotalData {
    /// Total number of transactions in the intergration.
    pub total_transactions: Option<u32>,
    /// Total of unique number of customers in the integration.
    pub unique_customers: Option<u32>,
    /// Total volume of transaction in the integration.
    pub total_volume: Option<u32>,
    /// Total volume of transaction broken down by currency.
    pub total_volume_by_currency: Option<Vec<VolumeByCurrency>>,
    /// Total volume of pending transfers.
    pub pending_transfers: Option<u32>,
    /// Total volumn of pending transfer broken down by currency.
    pub pending_transfers_by_currency: Option<Vec<VolumeByCurrency>>,
}

/// Transaction volume by currecny.
#[derive(Debug, Deserialize)]
pub struct VolumeByCurrency {
    /// Currency code.
    pub currency: String,
    /// Amount in the lowest denomincation of the currency.
    pub amount: u32,
}

/// Represents the response of the export transaction.
#[derive(Debug, Serialize, Deserialize)]
pub struct ExportTransactionResponse {
    /// This lets you know if your request was succesful or not.
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
#[derive(Debug, Deserialize)]
pub struct PartialDebitTransactionResponse {
    /// This lets you know if your request was succesful or not.
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
    pub created_at: String,
    /// The last update timestamp of the percentage split.
    pub updated_at: String,
    /// The list of subaccounts involved in the percentage split.
    pub subaccounts: Vec<SubaccountData>,
    /// The total count of subaccounts in the percentage split.
    pub total_subaccounts: u32,
}

/// Represents the data of th Subaccounts
#[derive(Debug, Deserialize, Serialize)]
pub struct SubaccountData {
    /// Sub account data
    pub subaccount: SubaccountResponse,
    /// Share of split assigned to this sub
    pub share: u32,
}

/// Represents a subaccount in the percentage split data.
#[derive(Debug, Deserialize, Serialize)]
pub struct SubaccountResponse {
    /// The ID of the subaccount.
    pub id: u32,
    /// The code of the subaccount.
    pub subaccount_code: String,
    /// The name of the business associated with the subaccount.
    pub business_name: String,
    /// The description of the business associated with the subaccount.
    pub description: String,
    /// The name of the primary contact for the business, if available.
    pub primary_contact_name: Option<String>,
    /// The email of the primary contact for the business, if available.
    pub primary_contact_email: Option<String>,
    /// The phone number of the primary contact for the business, if available.
    pub primary_contact_phone: Option<String>,
    /// Additional metadata associated with the subaccount, if available.
    pub metadata: Option<String>,
    /// The percentage charge for transactions associated with the subaccount.
    pub percentage_charge: u32,
    /// The name of the settlement bank for the subaccount.
    pub settlement_bank: String,
    /// The account number of the subaccount.
    pub account_number: String,
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

/// Represents the JSON response of the Paystack API when there is no data property
#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseWithoutData {
    /// The status of the JSON response.
    pub status: bool,
    /// The message associated with the JSON response.
    pub message: String,
}
