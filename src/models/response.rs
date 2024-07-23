//! response
//! ========
//! Holds the generic response templates for the API
use serde::{Deserialize, Serialize};

/// Generic response body template for the API
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Response<T> {
    /// This lets you know if your request was successful or not.
    pub status: bool,
    /// This is a summary of the response and its status.
    pub message: String,
    /// This contains the result of your request
    pub data: T,
}

/// The Meta object is used to provide context for the contents of the data key.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Meta {
    /// This is the total number of transactions that were performed by the customer.
    pub total: u8,
    /// This is the number of records skipped before the first record in the array returned.
    pub skipped: u8,
    /// This is the maximum number of records that will be returned per request.
    pub per_page: u8,
    /// This is the current page being returned.
    pub page: u8,
    /// This is how many pages in total are available for retrieval considering the maximum records per page specified.
    pub page_count: u8,
}
