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
    /// This contains the meta data associated with the response.
    pub meta: MetaData,
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

/// This struct represents the mata data of the response.
#[derive(Deserialize, Debug, Clone)]
pub struct MetaData {
    /// This is the total number of transactions that were performed by the customer.
    pub total: Option<u32>,
    /// This is the number of records skipped before the first record in the array returned.
    pub skipped: Option<u32>,
    /// This is the maximum number of records that will be returned per request.
    /// This can be modified by passing a new value as a perPage query parameter. Default: 50
    pub per_page: Option<u32>,
    /// This is the current page being returned.
    /// This is dependent on what page was requested using the page query parameter.
    ///
    /// `Default: 1`
    pub page: Option<u32>,
    /// This is how many pages in total are available for retrieval considering the maximum records per page specified.
    /// For context, if there are 51 records and perPage is left at its default value, pageCount will have a value of 2.
    pub page_count: Option<u32>,
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
