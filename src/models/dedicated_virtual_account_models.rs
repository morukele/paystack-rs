use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::{Currency, CustomerResponseData};

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
pub struct DedicatedVirtualAccountRequest {
    /// Customer ID or Code
    pub customer: String,
    /// The bank slug for preferred bank. To get a list of available banks, use the List Providers endpoint.
    #[builder(setter(strip_option), default)]
    pub preferred_bank: Option<String>,
    /// Subaccount code of the account you want to split the transaction with
    #[builder(setter(strip_option), default)]
    pub subaccount: Option<String>,
    /// Split code consisting of the lists of accounts you want to split the transaction with
    #[builder(setter(strip_option), default)]
    pub split_code: Option<String>,
    /// Customer's first name
    #[builder(setter(strip_option), default)]
    pub first_name: Option<String>,
    /// Customer's last name
    #[builder(setter(strip_option), default)]
    pub last_name: Option<String>,
    /// Customer's phone number
    #[builder(setter(strip_option), default)]
    pub phone: Option<String>,
    /// Customer's email address
    #[builder(setter(strip_option), default)]
    pub email: Option<String>,
    /// Currently accepts NG and GH only
    #[builder(setter(strip_option), default)]
    pub country: Option<String>,
    /// Customer's account number
    #[builder(setter(strip_option), default)]
    pub account_number: Option<String>,
    /// Customer's Bank Verification Number (Nigeria only)
    #[builder(setter(strip_option), default)]
    pub bvn: Option<String>,
    /// Customer's bank code
    #[builder(setter(strip_option), default)]
    pub bank_code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DedicatedVirtualAccountResponseData {
    pub bank: Option<Bank>,
    pub account_name: String,
    pub account_number: String,
    pub assigned: bool,
    pub currency: Currency,
    pub metadata: Option<String>,
    pub active: bool,
    pub id: u64,
    pub created_at: String,
    pub updated_at: String,
    pub assignment: Assignment,
    pub customer: Option<CustomerResponseData>,
    pub split_config: Option<SplitConfig>,
}

#[derive(Debug, Clone, Serialize, Default, Deserialize)]
pub struct SplitConfig {
    pub split_code: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Bank {
    pub name: String,
    pub id: u64,
    pub slug: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct Assignment {
    pub integration: u64,
    pub assignee_id: u64,
    pub assignee_type: String,
    pub expired: bool,
    pub account_type: String,
    pub assinged_at: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BankProviderData {
    pub provider_slug: String,
    pub bank_id: u64,
    pub bank_name: String,
    pub id: u64,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default, Builder)]
pub struct ListDedicatedAccountFilter {
    /// Status of the dedicated virtual account
    #[builder(setter(strip_option), default)]
    pub active: Option<bool>,
    /// The currency of the dedicated virtual account.
    #[builder(setter(strip_option), default)]
    pub currency: Option<Currency>,
    /// The bank's slug in lowercase, without spaces.
    #[builder(setter(strip_option), default)]
    pub provider_slug: Option<String>,
    /// The bank's ID
    #[builder(setter(strip_option), default)]
    pub bank_id: Option<String>,
    /// The customer's ID
    #[builder(setter(strip_option), default)]
    pub customer: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize, Default, Builder)]
pub struct SplitDedicatedAccountTransactionRequest {
    /// Customer ID or code
    pub customer: String,
    /// Subaccount code of the account you want to split the transaction with
    #[builder(setter(strip_option), default)]
    pub subaccount: Option<String>,
    /// Split code consisting of the lists of accounts you want to split the transaction with
    #[builder(setter(strip_option), default)]
    pub split_code: Option<String>,
    /// The bank slug for preferred bank. To get a list of available banks, use the List Providers endpoint
    #[builder(setter(strip_option), default)]
    pub preferred_bank: Option<String>,
}
