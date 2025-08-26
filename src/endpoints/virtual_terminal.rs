//! Virtual Terminal
//! ================
//! The Virtual Terminal API allows you to accept in-person payments without a POS device.

use std::{marker::PhantomData, sync::Arc};

use serde_json::json;

use crate::{
    DestinationRequest, DestinationResponse, HttpClient, PaystackAPIError, PaystackResult,
    Response, TransactionSplitResponseData, VirtualTerminalRequestData,
    VirtualTerminalResponseData, VirtualTerminalStatus,
};

#[derive(Debug, Clone)]
pub struct VirtualTerminalEndpoints<T: HttpClient + Default> {
    /// Paystack API key
    key: String,
    /// Base URL for the transaction route
    base_url: String,
    /// Http client for the route
    http: Arc<T>,
}

impl<T: HttpClient + Default> VirtualTerminalEndpoints<T> {
    /// Creates a new VirtualTerminalEndpoints instance
    ///
    /// # Arguments
    /// * `key` - The Paystack API key
    /// * `http` - The HTTP client implementation to use for API requests
    ///
    /// # Returns
    /// A new VirtualTerminalEndpoints instance
    pub fn new(key: Arc<String>, http: Arc<T>) -> VirtualTerminalEndpoints<T> {
        let base_url = String::from("https://api.paystack.co/virtual_terminal");
        VirtualTerminalEndpoints {
            key: key.to_string(),
            base_url,
            http,
        }
    }

    /// Creates a virtual terminal on your integration
    ///
    /// # Arguments
    /// * `virtual_terminal_request` - The request data to create the virtual terminal.
    ///   It should be created with the `VirtualTerminalRequestDataBuilder` struct.
    ///
    /// # Returns
    /// A Result containing the virtual terminal response data or an error
    pub async fn create_virtual_terminal(
        &self,
        virtual_terminal_request: VirtualTerminalRequestData,
    ) -> PaystackResult<VirtualTerminalResponseData> {
        let url = format!("{}", self.base_url);
        let body = serde_json::to_value(virtual_terminal_request)
            .map_err(|e| PaystackAPIError::VirtualTerminal(e.to_string()))?;

        let response = self
            .http
            .post(&url, &self.key, &body)
            .await
            .map_err(|e| PaystackAPIError::VirtualTerminal(e.to_string()))?;

        let parsed_response: Response<VirtualTerminalResponseData> =
            serde_json::from_str(&response)
                .map_err(|e| PaystackAPIError::VirtualTerminal(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Lists virtual terminals available on your integration
    ///
    /// # Arguments
    /// * `status` - Filter terminal by status
    /// * `per_page` - Number of records per page
    ///
    /// # Returns
    /// A Result containing a vector of virtual terminal response data or an error
    pub async fn list_virtual_terminals(
        &self,
        status: VirtualTerminalStatus,
        per_page: i32,
    ) -> PaystackResult<Vec<VirtualTerminalResponseData>> {
        let url = format!("{}", self.base_url);
        let status = status.to_string();
        let per_page = per_page.to_string();

        let query = vec![("status", status.as_str()), ("perPage", per_page.as_str())];

        let response = self
            .http
            .get(&url, &self.key, Some(&query))
            .await
            .map_err(|e| PaystackAPIError::VirtualTerminal(e.to_string()))?;

        let parsed_response: Response<Vec<VirtualTerminalResponseData>> =
            serde_json::from_str(&response)
                .map_err(|e| PaystackAPIError::VirtualTerminal(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Gets details of a virtual terminal on your integration
    ///
    /// # Arguments
    /// * `code` - Code of the virtual terminal to fetch
    ///
    /// # Returns
    /// A Result containing the virtual terminal response data or an error
    pub async fn fetch_virtual_terminal(
        self,
        code: String,
    ) -> PaystackResult<VirtualTerminalResponseData> {
        let url = format!("{}/{}", self.base_url, code);

        let response = self
            .http
            .get(&url, &self.key, None)
            .await
            .map_err(|e| PaystackAPIError::VirtualTerminal(e.to_string()))?;

        let parsed_response: Response<VirtualTerminalResponseData> =
            serde_json::from_str(&response)
                .map_err(|e| PaystackAPIError::VirtualTerminal(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Updates a virtual terminal on your integration
    ///
    /// # Arguments
    /// * `code` - Code of the virtual terminal to update
    /// * `name` - New name for the virtual terminal
    ///
    /// # Returns
    /// A Result containing the response or an error
    pub async fn update_virtual_terminal(
        &self,
        code: String,
        name: String,
    ) -> PaystackResult<PhantomData<String>> {
        let url = format!("{}/{}", self.base_url, code);
        let body = json!({
            "name": name
        });

        let response = self
            .http
            .put(&url, &self.key, &body)
            .await
            .map_err(|e| PaystackAPIError::VirtualTerminal(e.to_string()))?;

        let parsed_response: Response<PhantomData<String>> = serde_json::from_str(&response)
            .map_err(|e| PaystackAPIError::VirtualTerminal(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Deactivates a virtual terminal on your integration
    ///
    /// # Arguments
    /// * `code` - Code of the virtual terminal to deactivate
    ///
    /// # Returns
    /// A Result containing the response or an error
    pub async fn deactivate_virtual_terminal(
        &self,
        code: String,
    ) -> PaystackResult<PhantomData<String>> {
        let url = format!("{}/{}/deactivate", self.base_url, code);
        let body = json!({}); // empty body cause the route takes none

        let response = self
            .http
            .put(&url, &self.key, &body)
            .await
            .map_err(|e| PaystackAPIError::VirtualTerminal(e.to_string()))?;

        let parsed_response: Response<PhantomData<String>> = serde_json::from_str(&response)
            .map_err(|e| PaystackAPIError::VirtualTerminal(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Adds a WhatsApp destination number to a virtual terminal
    ///
    /// # Arguments
    /// * `code` - Code of the virtual terminal
    /// * `destinations` - Vector of destination requests containing notification recipients
    ///
    /// # Returns
    /// A Result containing a vector of destination responses or an error
    pub async fn assign_virtual_terminal_destination(
        &self,
        code: String,
        destinations: Vec<DestinationRequest>,
    ) -> PaystackResult<Vec<DestinationResponse>> {
        let url = format!("{}/{}/destination/assign", self.base_url, code);
        let body = json!({
            "destinations": destinations
        });

        let response = self
            .http
            .post(&url, &self.key, &body)
            .await
            .map_err(|e| PaystackAPIError::VirtualTerminal(e.to_string()))?;

        let parsed_response: Response<Vec<DestinationResponse>> =
            serde_json::from_str(&response)
                .map_err(|e| PaystackAPIError::VirtualTerminal(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Removes a WhatsApp destination number from a virtual terminal
    ///
    /// # Arguments
    /// * `code` - Code of the virtual terminal
    /// * `targets` - Vector of destination targets to unassign
    ///
    /// # Returns
    /// A Result containing the response or an error
    pub async fn unassign_virtual_terminal_destination(
        &self,
        code: String,
        targets: Vec<String>,
    ) -> PaystackResult<PhantomData<String>> {
        let url = format!("{}/{}/destination/unassign", self.base_url, code);
        let body = json!({
            "targets": targets
        });

        let response = self
            .http
            .post(&url, &self.key, &body)
            .await
            .map_err(|e| PaystackAPIError::VirtualTerminal(e.to_string()))?;

        let parsed_response: Response<PhantomData<String>> = serde_json::from_str(&response)
            .map_err(|e| PaystackAPIError::VirtualTerminal(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Adds a split payment code to a virtual terminal
    ///
    /// # Arguments
    /// * `code` - Code of the virtual terminal
    /// * `split_code` - Split code to add
    ///
    /// # Returns
    /// A Result containing the transaction split response data or an error
    pub async fn add_split_code_to_virtual_terminal(
        &self,
        code: String,
        split_code: String,
    ) -> PaystackResult<TransactionSplitResponseData> {
        let url = format!("{}/{}/split_code", self.base_url, code);
        let body = json!({
            "split_code": split_code
        });

        let response = self
            .http
            .put(&url, &self.key, &body)
            .await
            .map_err(|e| PaystackAPIError::VirtualTerminal(e.to_string()))?;

        let parsed_response: Response<TransactionSplitResponseData> =
            serde_json::from_str(&response)
                .map_err(|e| PaystackAPIError::VirtualTerminal(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Removes a split payment code from a virtual terminal
    ///
    /// # Arguments
    /// * `code` - Code of the virtual terminal
    /// * `split_code` - Split code to remove
    ///
    /// # Returns
    /// A Result containing the response or an error
    pub async fn remove_split_code_from_virtual_terminal(
        &self,
        code: String,
        split_code: String,
    ) -> PaystackResult<PhantomData<String>> {
        let url = format!("{}/{}/split_code", self.base_url, code);
        let body = json!({
            "split_code": split_code
        });

        let response = self
            .http
            .delete(&url, &self.key, &body)
            .await
            .map_err(|e| PaystackAPIError::VirtualTerminal(e.to_string()))?;

        let parsed_response: Response<PhantomData<String>> = serde_json::from_str(&response)
            .map_err(|e| PaystackAPIError::VirtualTerminal(e.to_string()))?;

        Ok(parsed_response)
    }
}
