//! response
//! ========
//! Holds the generic response templates for the API
use crate::utils::string_or_number_to_u16;
use serde::{Deserialize, Serialize};

/// Generic response body template for the API
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Response<T> {
    /// This lets you know if your request was successful or not.
    pub status: bool,
    /// This is a summary of the response and its status.
    pub message: String,
    /// This contains the result of your request
    #[serde(default)]
    pub data: T,
    /// This contains meta data object
    pub meta: Option<Meta>,
}

/// The Meta object is used to provide context for the contents of the data key.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    /// This is the total number of transactions that were performed by the customer.
    #[serde(deserialize_with = "string_or_number_to_u16")]
    pub total: u16,
    /// This is the number of records skipped before the first record in the array returned.
    #[serde(deserialize_with = "string_or_number_to_u16")]
    pub skipped: u16,
    /// This is the maximum number of records that will be returned per request.
    #[serde(deserialize_with = "string_or_number_to_u16")]
    pub per_page: u16,
    /// This is the current page being returned.
    #[serde(deserialize_with = "string_or_number_to_u16")]
    pub page: u16,
    /// This is how many pages in total are available for retrieval considering the maximum records per page specified.
    #[serde(deserialize_with = "string_or_number_to_u16")]
    pub page_count: u16,
}
