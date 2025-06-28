use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::{Currency, CustomerResponseData};

#[derive(Debug, Clone, Serialize, Deserialize, Default, Builder)]
pub struct CreateDedicatedVirtualAccountRequest {
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
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DedicatedVirtualAccountResponseData {
    pub bank: Bank,
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
    pub customer: CustomerResponseData,
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
