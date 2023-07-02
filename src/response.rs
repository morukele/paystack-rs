use serde::{Deserialize, Serialize};

/// This struct represents the response of the Paystack transaction initalization.
#[derive(Deserialize, Debug)]
pub struct TransactionResponse {
    pub status: bool,
    pub message: String,
    pub data: TransactionResponseData,
}

/// This struct represents the data of the transaction response
#[derive(Deserialize, Debug)]
pub struct TransactionResponseData {
    pub authorization_url: String,
    pub access_code: String,
    pub reference: String,
}

/// This struct represents the transaction status response
#[derive(Deserialize, Debug)]
pub struct TransactionStatus {
    pub status: bool,
    pub message: String,
    pub data: TransactionStatusData,
}

/// This struct represents a list of transaction status
#[derive(Deserialize, Debug)]
pub struct TransactionStatusList {
    pub status: bool,
    pub message: String,
    pub data: Vec<TransactionStatusData>,
}

/// This struct represents the data of the transaction status response
#[derive(Deserialize, Debug)]
pub struct TransactionStatusData {
    pub id: Option<u64>,
    pub status: Option<String>,
    pub reference: Option<String>,
    pub amount: Option<u64>,
    pub message: Option<String>,
    pub gateway_response: Option<String>,
    pub paid_at: Option<String>,
    pub created_at: Option<String>,
    pub channel: Option<String>,
    pub currency: Option<String>,
    pub ip_address: Option<String>,
    pub metadata: Option<String>,
    pub fees: Option<i32>,
    pub customer: Option<Customer>,
}

/// This struct represents the authorization data of the transaction status response
#[derive(Debug, Deserialize)]
pub struct Authorization {
    pub authorization_code: Option<String>,
    pub bin: Option<String>,
    pub last4: Option<String>,
    pub exp_month: Option<String>,
    pub exp_year: Option<String>,
    pub channel: Option<String>,
    pub card_type: Option<String>,
    pub bank: Option<String>,
    pub country_code: Option<String>,
    pub brand: Option<String>,
    pub reusable: Option<bool>,
    pub signature: Option<String>,
    pub account_name: Option<String>,
}

/// This struct represents the Paystack customer data
#[derive(Debug, Deserialize, Serialize)]
pub struct Customer {
    pub id: Option<u64>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: Option<String>,
    pub customer_code: String,
    pub phone: Option<String>,
    pub metadata: Option<String>,
    pub risk_action: Option<String>,
    pub international_format_phone: Option<String>,
}
