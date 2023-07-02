use serde::{Deserialize, Serialize};

/// This struct mirrors the response of the Paystack transaction initalization response.
///
/// It contains the following properties:
///
/// ```json
/// {
///     "status": Boolean,
///     "message": String,
///     "data": TransactionResponseData
/// }
/// ```
#[derive(Deserialize, Debug)]
pub struct TransactionResponse {
    pub status: bool,
    pub message: String,
    pub data: TransactionResponseData,
}

/// This struct mirrors the transaction response data dictionary.
///
/// It contains the following properties:
///
/// ```json
/// {
///     "authroization_url": String,
///     "access_code": String,
///     "reference": String
/// }
/// ```
#[derive(Deserialize, Debug)]
pub struct TransactionResponseData {
    pub authorization_url: String,
    pub access_code: String,
    pub reference: String,
}

#[derive(Deserialize, Debug)]
pub struct TransactionStatus {
    pub status: bool,
    pub message: String,
    pub data: TransactionStatusData,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TransactionStatusData {
    pub id: u64,
    pub status: String,
    pub reference: String,
    pub amount: u64,
    pub message: Option<String>,
    pub gateway_response: String,
    pub paid_at: Option<String>,
    pub created_at: String,
    pub channel: String,
    pub currency: String,
    pub ip_address: String,
    pub metadata: String,
    pub fees: Option<i32>,
}
