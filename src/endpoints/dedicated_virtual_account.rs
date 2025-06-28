//! Dedicated Virtual Account
//! =========================
//! The Dedicated Virtual Account API enables Nigerian and Ghanaian merchants to manage unique payment accounts of their customers.

use std::sync::Arc;

use crate::{
    http, CreateDedicatedVirtualAccountRequest, DedicatedVirtualAccountResponseData, HttpClient,
    PaystackAPIError, PaystackResult, Response,
};

#[derive(Debug, Clone)]
pub struct DedicatedVirtualAccountEndpoints<T: HttpClient + Default> {
    key: String,
    base_url: String,
    http: Arc<T>,
}

impl<T: HttpClient + Default> DedicatedVirtualAccountEndpoints<T> {
    pub fn new(key: Arc<String>, http: Arc<T>) -> DedicatedVirtualAccountEndpoints<T> {
        let base_url = String::from("https://api.paystack.co/dedicated_account");
        DedicatedVirtualAccountEndpoints {
            key: key.to_string(),
            base_url,
            http,
        }
    }

    /// Create a dedicated virtual account for an existing customer.
    ///
    /// Takes in the following:
    ///     - `create_dedicated_virtual_account_request`: The request data to create the dedicated virtual account for the customer.
    /// It should be created with the `CreateDedicatedVirtualAccountRequstBuilder` struct.
    pub async fn create_dedicated_virtual_account(
        &self,
        create_dedicated_virtual_account_request: CreateDedicatedVirtualAccountRequest,
    ) -> PaystackResult<DedicatedVirtualAccountResponseData> {
        let url = format!("{}", self.base_url);
        let body = serde_json::to_value(create_dedicated_virtual_account_request)
            .map_err(|e| PaystackAPIError::DedicatedVirtualAccount(e.to_string()))?;

        let response = self.http.post(&url, &self.key, &body).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<DedicatedVirtualAccountResponseData> =
                    serde_json::from_str(&response)
                        .map_err(|e| PaystackAPIError::DedicatedVirtualAccount(e.to_string()))?;

                Ok(parsed_response)
            }
            Err(e) => Err(PaystackAPIError::DedicatedVirtualAccount(e.to_string())),
        }
    }
}
