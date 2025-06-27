use std::fmt;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::{Authorization, Subscription, TransactionStatusData};

/// This struct represents the Paystack customer data
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct CustomerResponseData {
    pub id: u64,
    pub integration: Option<u64>,
    pub domain: Option<String>,
    pub identified: Option<bool>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub email: String,
    pub customer_code: String,
    pub phone: Option<String>,
    pub risk_action: Option<String>,
    pub international_format_phone: Option<String>,
    pub identification: Option<String>,
    pub transactions: Option<Vec<TransactionStatusData>>,
    pub subscriptions: Option<Vec<Subscription>>,
    pub authorizations: Option<Vec<Authorization>>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
    pub total_transactions: Option<u16>,
    pub total_transaction_value: Option<Vec<String>>,
    pub dedicated_account: Option<String>,
}

/// This struct constains the data for creating a customer in your integration
#[derive(Debug, Clone, Serialize, Default, Deserialize, Builder)]
pub struct CreateCustomerRequest {
    /// Customer's email address
    pub email: String,
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

/// This struct constains the data for updating a customer in your integration
#[derive(Debug, Clone, Serialize, Default, Deserialize, Builder)]
pub struct UpdateCustomerRequest {
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

#[derive(Debug, Clone, Serialize, Default, Deserialize, Builder)]
pub struct ValidateCustomerRequest {
    /// Customer's first name
    pub first_name: String,
    /// Customer's last name
    pub last_name: String,
    /// Customer's middle name
    #[builder(setter(strip_option), default)]
    pub middle_name: Option<String>,
    /// Predefined types of identification. Only `bank_code` is supported at the moment
    #[serde(rename = "type")]
    pub identification_type: IdentificationType,
    /// Customer's identification number
    #[builder(setter(strip_option), default)]
    pub value: Option<String>,
    /// 2 letter country code of identification issuer
    pub country: String,
    /// Customer's Bank Verification Number
    pub bvn: String,
    /// Customer bank code
    pub bank_code: String,
    /// Customer's bank account number. (required if `identification_type` is `bank_account`.
    #[builder(setter(strip_option), default)]
    pub account_number: Option<String>,
}

/// Represents the different predefined types of identification.
///
/// Only `bank_account`is supported at the moment.
#[derive(Debug, Serialize, Default, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum IdentificationType {
    #[default]
    #[serde(rename = "bank_account")]
    BankAccount,
}

impl fmt::Display for IdentificationType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let identification_type = match self {
            IdentificationType::BankAccount => "bank_account",
        };
        write!(f, "{}", identification_type)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_build_customer() {
        let customer = CreateCustomerRequestBuilder::default()
            .email("customer@example.com".to_string())
            .first_name("Zero".to_string())
            .last_name("Sum".to_string())
            .phone("+2348123456789".to_string())
            .build()
            .expect("unable to build customer request");

        assert_eq!(customer.first_name, Some("Zero".to_string()));
        assert_eq!(customer.last_name, Some("Sum".to_string()));
    }

    #[test]
    fn build_customer_with_invalid_data_fails() {
        let first_name = "Zero".to_string();
        let last_name = "Sum".to_string();
        let phone = "+2348123456789".to_string();

        let body = CreateCustomerRequestBuilder::default()
            .first_name(first_name)
            .last_name(last_name)
            .phone(phone)
            .build();

        assert!(body.is_err());
    }

    #[test]
    fn can_use_identification_type() {
        let identification = IdentificationType::BankAccount;

        assert_eq!(identification.to_string(), "bank_account".to_string());
    }
}
