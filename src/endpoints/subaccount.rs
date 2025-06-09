//! Subaccounts
//! ===========
//! The Subaccounts API allows you to create and manage subaccounts on your integration.
//! Subaccounts can be used to split payment between two accounts (your main account and a subaccount).

use crate::{
    HttpClient, PaystackAPIError, PaystackResult, Response, SubaccountRequest,
    SubaccountsResponseData,
};
use std::sync::Arc;

/// A struct to hold all functions in the subaccount API route
#[derive(Debug, Clone)]
pub struct SubaccountEndpoints<T: HttpClient + Default> {
    key: String,
    base_url: String,
    http: Arc<T>,
}

impl<T: HttpClient + Default> SubaccountEndpoints<T> {
    /// Constructor
    pub fn new(key: Arc<String>, http: Arc<T>) -> SubaccountEndpoints<T> {
        let base_url = String::from("https://api.paystack.co/subaccount");
        SubaccountEndpoints {
            key: key.to_string(),
            base_url,
            http,
        }
    }

    /// Create a subaccount on your integration
    ///
    /// Takes in the following parameters
    ///     - body: subaccount to create `SubaccountRequest`; this is constructed using the
    ///      `SubaccountRequestBuilder`.
    pub async fn create_subaccount(
        &self,
        subaccount_request: SubaccountRequest,
    ) -> PaystackResult<SubaccountsResponseData> {
        let url = self.base_url.to_string();
        let body = serde_json::to_value(subaccount_request)
            .map_err(|e| PaystackAPIError::Subaccount(e.to_string()))?;

        let response = self.http.post(&url, &self.key, &body).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<SubaccountsResponseData> =
                    serde_json::from_str(&response)
                        .map_err(|e| PaystackAPIError::Subaccount(e.to_string()))?;
                Ok(parsed_response)
            }
            Err(e) => Err(PaystackAPIError::Subaccount(e.to_string())),
        }
    }
}
