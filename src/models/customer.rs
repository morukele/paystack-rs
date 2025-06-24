use serde::{Deserialize, Serialize};

/// This struct represents the Paystack customer data
#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Customer {
    /// Customer's Id.
    pub id: u32,
    /// Customer's first name.
    pub first_name: Option<String>,
    /// Customer's last name.
    pub last_name: Option<String>,
    /// Customer's email address.
    pub email: String,
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
