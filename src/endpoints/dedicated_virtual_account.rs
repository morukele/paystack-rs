//! Dedicated Virtual Account
//! =========================
//! The Dedicated Virtual Account API enables Nigerian and Ghanaian merchants to manage unique payment accounts of their customers.

use super::BASE_URL;
use crate::{
    BankProviderData, DedicatedVirtualAccountRequest, DedicatedVirtualAccountResponseData,
    HttpClient, ListDedicatedAccountFilter, PaystackAPIError, PaystackResult, Response,
    SplitDedicatedAccountTransactionRequest,
};
use serde_json::json;
use std::{marker::PhantomData, sync::Arc};

#[derive(Debug, Clone)]
pub struct DedicatedVirtualAccountEndpoints<T: HttpClient + Default> {
    key: String,
    base_url: String,
    http: Arc<T>,
}

/// Handles operations related to dedicated virtual accounts in the Paystack API
impl<T: HttpClient + Default> DedicatedVirtualAccountEndpoints<T> {
    /// Creates a new DedicatedVirtualAccountEndpoints instance
    ///
    /// # Arguments
    /// * `key` - The Paystack API key
    /// * `http` - The HTTP client implementation to use for API requests
    ///
    /// # Returns
    /// A new DedicatedVirtualAccountEndpoints instance
    pub fn new(key: Arc<String>, http: Arc<T>) -> DedicatedVirtualAccountEndpoints<T> {
        let base_url = format!("{}/dedicated_account", BASE_URL);
        DedicatedVirtualAccountEndpoints {
            key: key.to_string(),
            base_url,
            http,
        }
    }

    /// Create a dedicated virtual account for an existing customer.
    ///
    /// # Arguments
    /// * `create_dedicated_virtual_account_request` - The request data to create the dedicated virtual account for the customer.
    ///   It should be created with the `DedicatedVirtualAccountRequstBuilder` struct.
    ///
    /// # Returns
    /// A Result containing the dedicated virtual account response data or an error
    pub async fn create_dedicated_virtual_account(
        &self,
        create_dedicated_virtual_account_request: DedicatedVirtualAccountRequest,
    ) -> PaystackResult<DedicatedVirtualAccountResponseData> {
        let url = &self.base_url;
        let body = serde_json::to_value(create_dedicated_virtual_account_request)
            .map_err(|e| PaystackAPIError::DedicatedVirtualAccount(e.to_string()))?;

        let response = self
            .http
            .post(&url, &self.key, &body)
            .await
            .map_err(|e| PaystackAPIError::DedicatedVirtualAccount(e.to_string()))?;

        let parsed_response: Response<DedicatedVirtualAccountResponseData> =
            serde_json::from_str(&response)
                .map_err(|e| PaystackAPIError::DedicatedVirtualAccount(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Creates a customer, validates them and assigns a dedicated virtual account.
    ///
    /// # Arguments
    /// * `assign_dedicated_virtual_account_request` - The request data to assign the dedicated virtual account.
    ///   It should be created with the `DedicatedVirtualAccountRequestBuilder`
    ///
    /// # Returns
    /// A Result containing the response or an error
    pub async fn assign_dedicated_virtual_account(
        &self,
        assign_dedicated_virtual_account_request: DedicatedVirtualAccountRequest,
    ) -> PaystackResult<PhantomData<String>> {
        let url = &self.base_url;
        let body = serde_json::to_value(assign_dedicated_virtual_account_request)
            .map_err(|e| PaystackAPIError::DedicatedVirtualAccount(e.to_string()))?;

        let response = self
            .http
            .post(&url, &self.key, &body)
            .await
            .map_err(|e| PaystackAPIError::DedicatedVirtualAccount(e.to_string()))?;

        let parsed_response: Response<PhantomData<String>> = serde_json::from_str(&response)
            .map_err(|e| PaystackAPIError::DedicatedVirtualAccount(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Lists dedicated virtual accounts available on your integration.
    ///
    /// # Arguments
    /// * `filter` - Optional set of parameters to filter the dedicated accounts returned.
    ///   It should be created with the `ListDedicatedAccountFilterBuilder` struct.
    ///
    /// # Returns
    /// A Result containing a vector of dedicated virtual account response data or an error
    pub async fn list_dedicated_accounts(
        &self,
        filter: Option<ListDedicatedAccountFilter>,
    ) -> PaystackResult<Vec<DedicatedVirtualAccountResponseData>> {
        let url = &self.base_url;
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
        let response = self
            .http
            .get(&url, &self.key, Some(&query))
            .await
            .map_err(|e| PaystackAPIError::DedicatedVirtualAccount(e.to_string()))?;

        let parsed_response: Response<Vec<DedicatedVirtualAccountResponseData>> =
            serde_json::from_str(&response)
                .map_err(|e| PaystackAPIError::DedicatedVirtualAccount(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Gets details of a dedicated virtual account on your integration
    ///
    /// # Arguments
    /// * `dedicated_account_id` - ID of dedicated virtual account to fetch
    ///
    /// # Returns
    /// A Result containing the dedicated virtual account response data or an error
    pub async fn fetch_dedicated_virtual_account(
        &self,
        dedicated_account_id: u64,
    ) -> PaystackResult<DedicatedVirtualAccountResponseData> {
        let url = format!("{}/{}", self.base_url, dedicated_account_id);

        let response = self
            .http
            .get(&url, &self.key, None)
            .await
            .map_err(|e| PaystackAPIError::DedicatedVirtualAccount(e.to_string()))?;

        let parsed_response: Response<DedicatedVirtualAccountResponseData> =
            serde_json::from_str(&response)
                .map_err(|e| PaystackAPIError::DedicatedVirtualAccount(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Requery Dedicated Virtual Account for new transactions
    ///
    /// # Arguments
    /// * `account_number` - Virtual account number to requery
    /// * `provider_slug` - The bank's slug in lowercase, without spaces
    /// * `date` - Optional day the transfer was made in YYYY-MM-DD format
    ///
    /// # Returns
    /// A Result containing the response or an error
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

        let response = self
            .http
            .get(&url, &self.key, Some(&query))
            .await
            .map_err(|e| PaystackAPIError::DedicatedVirtualAccount(e.to_string()))?;

        let parsed_response: Response<PhantomData<String>> = serde_json::from_str(&response)
            .map_err(|e| PaystackAPIError::DedicatedVirtualAccount(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Deactivate a dedicated virtual account on your integration
    ///
    /// # Arguments
    /// * `dedicated_account_id` - ID of dedicated virtual account to deactivate
    ///
    /// # Returns
    /// A Result containing the dedicated virtual account response data or an error
    pub async fn deactivate_dedicated_account(
        &self,
        dedicated_account_id: u64,
    ) -> PaystackResult<DedicatedVirtualAccountResponseData> {
        let url = format!("{}/{}", self.base_url, dedicated_account_id);
        let body = json!({}); // empty body since the route takes none.

        let response = self
            .http
            .delete(&url, &self.key, &body)
            .await
            .map_err(|e| PaystackAPIError::DedicatedVirtualAccount(e.to_string()))?;

        let parsed_response: Response<DedicatedVirtualAccountResponseData> =
            serde_json::from_str(&response)
                .map_err(|e| PaystackAPIError::DedicatedVirtualAccount(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Split a dedicated virtual account transaction with one or more accounts.
    ///
    /// # Arguments
    /// * `split_dedocated_account_transaction_request` - The request data to split a transaction.
    ///   It should be created with the `SplitDedicatedAccountTransactionRequestBuilder` struct.
    ///
    /// # Returns
    /// A Result containing the dedicated virtual account response data or an error

    pub async fn split_dedicated_account_transaction(
        &self,
        split_dedocated_account_transaction_request: SplitDedicatedAccountTransactionRequest,
    ) -> PaystackResult<DedicatedVirtualAccountResponseData> {
        let url = &self.base_url;
        let body = serde_json::to_value(split_dedocated_account_transaction_request)
            .map_err(|e| PaystackAPIError::DedicatedVirtualAccount(e.to_string()))?;

        let response = self
            .http
            .post(&url, &self.key, &body)
            .await
            .map_err(|e| PaystackAPIError::DedicatedVirtualAccount(e.to_string()))?;

        let parsed_response: Response<DedicatedVirtualAccountResponseData> =
            serde_json::from_str(&response)
                .map_err(|e| PaystackAPIError::DedicatedVirtualAccount(e.to_string()))?;

        Ok(parsed_response)
    }

    /// If you've previously set up split payment for transactions on a dedicated virtual account, you can remove it with this endpoint
    ///
    /// # Arguments
    /// * `account_number` - The account number of the dedicated virtual account to remove split from
    ///
    /// # Returns
    /// A Result containing the dedicated virtual account response data or an error
    pub async fn remove_split_from_dedicated_account(
        &self,
        account_number: String,
    ) -> PaystackResult<DedicatedVirtualAccountResponseData> {
        let url = &self.base_url;
        let body = json!({
            "account_number": account_number
        });

        let response = self
            .http
            .delete(&url, &self.key, &body)
            .await
            .map_err(|e| PaystackAPIError::DedicatedVirtualAccount(e.to_string()))?;

        let parsed_response: Response<DedicatedVirtualAccountResponseData> =
            serde_json::from_str(&response)
                .map_err(|e| PaystackAPIError::DedicatedVirtualAccount(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Get available bank providers for a dedicated virtual account
    ///
    /// # Arguments
    /// None
    ///
    /// # Returns
    /// A Result containing a vector of bank provider data or an error
    pub async fn fetch_bank_providers(&self) -> PaystackResult<Vec<BankProviderData>> {
        let url = format!("{}/available_providers", self.base_url);

        let response = self
            .http
            .get(&url, &self.key, None)
            .await
            .map_err(|e| PaystackAPIError::DedicatedVirtualAccount(e.to_string()))?;

        let parsed_response: Response<Vec<BankProviderData>> = serde_json::from_str(&response)
            .map_err(|e| PaystackAPIError::DedicatedVirtualAccount(e.to_string()))?;

        Ok(parsed_response)
    }
}
