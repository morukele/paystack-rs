//! Customers
//! ==========
//! This file contains the models for working with the customers endpoint.

use std::collections::HashMap;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// This struct is used to create a customer body for creating a new customer using the Paystack API.
/// This struct should be created using the `CreateCustomerBodyBuilder`.
#[derive(Serialize, Debug, Default, Builder)]
pub struct CreateCustomerBody {
    /// Customer's email address.
    email: String,
    /// Customer's first name.
    first_name: String,
    /// Customer's last name.
    last_name: String,
    /// Customer's phone number.
    phone: String,
    /// A set of key/value pairs that you can attach to the customer. It can be used to store additional information in a structured format.
    #[builder(default = "None")]
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<HashMap<String, String>>,
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

/// This struct represents the response from the create customer endpoint.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateCustomerResponse {
    /// This lets you know if your request was successful or not.
    pub status: bool,
    /// This is a summary of the response and its status.
    pub message: String,
    /// This contains the results of your request.
    pub data: CreateCustomerResponseData,
}

/// This struct represents the data in the create customer response.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CreateCustomerResponseData {
    /// The customer email address.
    pub email: String,
    /// The integration id the customer is registered to.
    pub integration: i32,
    /// The domain of the integration (test or production).
    pub domain: String,
    /// The customer code.
    pub customer_code: String,
    /// The customer id.
    pub id: i32,
    /// Status of customer identification.
    pub identified: bool,
    /// The identification used.
    pub identifications: Option<String>,
    /// Customer creation time.
    #[serde(rename = "updatedAt")]
    pub created_at: Option<String>,
    /// Last customer update time.
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
}

/// This struct represents the response from the list customer route.
pub struct ListCustomerResponse {

}
