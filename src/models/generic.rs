//! Generic Model
//! ==========
//! This file contains generic models for the Paystack API.

use serde::{Deserialize, Serialize};

/// MetaData of list response
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct MetaData {
    /// This is the total number of transactions that were performed by the customer.
    pub total: u32,
    /// This is the number of records skipped before the first record in the array returned.
    pub skipped: u32,
    /// This is the maximum number of records that will be returned per request.
    #[serde(rename = "perPage")]
    pub per_page: String,
    /// This is the current `page` being returned.
    pub page: u32,
    /// This is how many pages in total are available for retrieval considering the maximum records per page specified.
    #[serde(rename = "pageCount")]
    pub page_count: u32,
}

/// This struct represents the authorization data of the transaction status response
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Authorization {
    /// Authorization code generated for the Transaction.
    pub authorization_code: Option<String>,
    /// Bin number for Transaction authorization.
    pub bin: Option<String>,
    /// Last 4 digits of authorized card.
    pub last4: Option<String>,
    /// Authorized card expiry month.
    pub exp_month: Option<String>,
    /// Authorized card expiry year.
    pub exp_year: Option<String>,
    /// Authorization channel. It could be `card` or `bank`.
    pub channel: Option<String>,
    /// Type of card used in the Authorization
    pub card_type: Option<String>,
    /// Name of bank associated with the Authorization.
    pub bank: Option<String>,
    /// Country code of the Authorization.
    pub country_code: Option<String>,
    /// Brand of of the Authorization if it is a card.
    pub brand: Option<String>,
    /// Specifies if the Authorization is reusable.
    pub reusable: Option<bool>,
    /// Signature of the Authorization.
    pub signature: Option<String>,
    /// Name of the account associated with the authorization.
    pub account_name: Option<String>,
}
