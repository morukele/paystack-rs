//! Dedicated Virtual Account
//! =========================
//! The Dedicated Virtual Account API enables Nigerian and Ghanaian merchants to manage unique payment accounts of their customers.

use std::{marker::PhantomData, sync::Arc};

use crate::{
    DedicatedVirtualAccountRequest, DedicatedVirtualAccountResponseData, HttpClient,
    ListDedicatedAccountFilter, PaystackAPIError, PaystackResult, Response,
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
    ///
    /// Takes in the following:
    ///     - `filter`: an optional set of parameters to filter the dedicated accounts returned.
    /// It should be created with the `ListDedicatedAccountFilterBuilder` struct.
    pub async fn list_dedicated_accounts(
        &self,
        filter: Option<ListDedicatedAccountFilter>,
    ) -> PaystackResult<Vec<DedicatedVirtualAccountResponseData>> {
        let url = format!("{}", self.base_url);
        let mut query = vec![];
        // Build the query vec with the value in the filter struct
        if let Some(filter) = filter {
            if let Some(active) = filter.active {
                query.push(("active", active.to_string()));
            }
            if let Some(currency) = filter.currency {
                query.push(("currency", currency.to_string()));
            }
            if let Some(provider_slug) = filter.provider_slug {
                query.push(("provider_slug", provider_slug));
            }
            if let Some(bank_id) = filter.bank_id {
                query.push(("bank_id", bank_id));
            }
            if let Some(customer) = filter.customer {
                query.push(("customer", customer));
            }
        }

        // Transform String to &str using iter
        let query: Vec<(&str, &str)> = query.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let response = self.http.get(&url, &self.key, Some(&query)).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<Vec<DedicatedVirtualAccountResponseData>> =
                    serde_json::from_str(&response)
                        .map_err(|e| PaystackAPIError::DedicatedVirtualAccount(e.to_string()))?;

                Ok(parsed_response)
            }
            Err(e) => Err(PaystackAPIError::DedicatedVirtualAccount(e.to_string())),
        }
    }

    /// Get details of a dedicated virtual account on your integration
    ///
    /// Takes in the following:
    ///     - `dedicated_account_id`: ID of dedicated virtual account
    pub async fn fetch_dedicated_virtual_account(
        &self,
        dedicated_account_id: u64,
    ) -> PaystackResult<DedicatedVirtualAccountResponseData> {
        let url = format!("{}/{}", self.base_url, dedicated_account_id);

        let response = self.http.get(&url, &self.key, None).await;

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

    /// Requery Dedicated Virtual Account for new transactions.
    ///
    /// Takes in the following:
    ///     - `account_number`: Virtual account number to requery
    ///     - `provider_slug`: The bank's slug in lowercase, without spaces.
    ///     - `date`: (Optional) The day the transfer was made in `YYYY-MM_DD` format.
    pub async fn requery_dedicated_account(
        &self,
        account_number: String,
        provider_slug: String,
        date: Option<String>,
    ) -> PaystackResult<PhantomData<String>> {
        let url = format!("{}/requery", self.base_url);
        let mut query = vec![
            ("account_number", account_number),
            ("provider_slug", provider_slug),
        ];
        if date.is_some() {
            query.push(("date", date.unwrap()));
        }

        // convert Vec<(&str, String)> to Vec<(&str, &str)>
        let query: Vec<(&str, &str)> = query.iter().map(|(k, v)| (*k, v.as_str())).collect();

        let response = self.http.get(&url, &self.key, Some(&query)).await;

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
}
