//! response
//! ========
//! Holds the generic response templates for the API
use crate::utils::option_string_or_number_to_u16;
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
    pub data: Option<T>,
    /// This contains meta data object
    pub meta: Option<Meta>,
    #[serde(rename = "type")]
    pub response_type: Option<String>,
    pub code: Option<String>,
}

/// The Meta object is used to provide context for the contents of the data key.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct Meta {
    /// This is the total number of transactions that were performed by the customer.
    #[serde(deserialize_with = "option_string_or_number_to_u16")]
    pub total: Option<u16>,
    /// This is the number of records skipped before the first record in the array returned.
    #[serde(deserialize_with = "option_string_or_number_to_u16")]
    pub skipped: Option<u16>,
    /// This is the maximum number of records that will be returned per request.
    #[serde(deserialize_with = "option_string_or_number_to_u16")]
    pub per_page: Option<u16>,
    /// This is the current page being returned.
    #[serde(deserialize_with = "option_string_or_number_to_u16")]
    pub page: Option<u16>,
    /// This is how many pages in total are available for retrieval considering the maximum records per page specified.
    #[serde(deserialize_with = "option_string_or_number_to_u16")]
    pub page_count: Option<u16>,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub next_step: Option<String>,
}
