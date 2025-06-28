//! Dedicated Virtual Account
//! =========================
//! The Dedicated Virtual Account API enables Nigerian and Ghanaian merchants to manage unique payment accounts of their customers.

use std::{marker::PhantomData, sync::Arc};

use crate::{
    DedicatedVirtualAccountRequest, DedicatedVirtualAccountResponseData, HttpClient,
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
    /// It should be created with the `DedicatedVirtualAccountRequstBuilder` struct.
    pub async fn create_dedicated_virtual_account(
        &self,
        create_dedicated_virtual_account_request: DedicatedVirtualAccountRequest,
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

    /// This function creates a customer, validates the customer and assigns a dedicated virtual account to the customer.
    ///
    /// It takes in the following:
    ///     - assign_dedicated_virtual_account_request: The request data to assign the dedicated virtual account.
    /// It should be created with the `DedicatedVirtualAccountRequestBuilder`
    pub async fn assign_dedicated_virtual_account(
        &self,
        assign_dedicated_virtual_account_request: DedicatedVirtualAccountRequest,
    ) -> PaystackResult<PhantomData<String>> {
        let url = format!("{}", self.base_url);
        let body = serde_json::to_value(assign_dedicated_virtual_account_request)
            .map_err(|e| PaystackAPIError::DedicatedVirtualAccount(e.to_string()))?;

        let response = self.http.post(&url, &self.key, &body).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<PhantomData<String>> =
                    serde_json::from_str(&response)
                        .map_err(|e| PaystackAPIError::DedicatedVirtualAccount(e.to_string()))?;

                Ok(parsed_response)
            }
            Err(e) => Err(PaystackAPIError::DedicatedVirtualAccount(e.to_string())),
        }
    }

    /// List dedicated virtual accounts available on your integration.
    pub async fn list_dedicated_accounts(
    ) -> PaystackResult<Vec<DedicatedVirtualAccountResponseData>> {
        todo!()
    }
}
