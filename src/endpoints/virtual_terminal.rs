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
    /// Constructor
    pub fn new(key: Arc<String>, http: Arc<T>) -> VirtualTerminalEndpoints<T> {
        let base_url = String::from("https://api.paystack.co/virtual_terminal");
        VirtualTerminalEndpoints {
            key: key.to_string(),
            base_url,
            http,
        }
    }

    /// Create a Virtual Terminal on your integration.
    ///
    /// Takes in the following:
    ///     - `VirtualTerminalRequestData`: The request data to create the virtual terminal. It is created with the `VirtualTerminalRequestDataBuilder` struct.
    pub async fn create_virtual_terminal(
        &self,
        virtual_terminal_request: VirtualTerminalRequestData,
    ) -> PaystackResult<VirtualTerminalResponseData> {
        let url = format!("{}", self.base_url);
        let body = serde_json::to_value(virtual_terminal_request)
            .map_err(|e| PaystackAPIError::VirtualTerminal(e.to_string()))?;

        let response = self.http.post(&url, &self.key, &body).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<VirtualTerminalResponseData> =
                    serde_json::from_str(&response)
                        .map_err(|e| PaystackAPIError::VirtualTerminal(e.to_string()))?;

                Ok(parsed_response)
            }
            Err(e) => Err(PaystackAPIError::VirtualTerminal(e.to_string())),
        }
    }

    /// List Virtual Terminals on your integration.
    ///
    /// Takes in the following:
    ///     - `status`: Filter terminal by status.
    ///     - `per_page`: Number of records per page.
    pub async fn list_virtual_terminals(
        &self,
        status: VirtualTerminalStatus,
        per_page: i32,
    ) -> PaystackResult<Vec<VirtualTerminalResponseData>> {
        let url = format!("{}", self.base_url);
        let status = status.to_string();
        let per_page = per_page.to_string();

        let query = vec![("status", status.as_str()), ("perPage", per_page.as_str())];

        let response = self.http.get(&url, &self.key, Some(&query)).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<Vec<VirtualTerminalResponseData>> =
                    serde_json::from_str(&response)
                        .map_err(|e| PaystackAPIError::VirtualTerminal(e.to_string()))?;

                Ok(parsed_response)
            }
            Err(e) => Err(PaystackAPIError::VirtualTerminal(e.to_string())),
        }
    }

    /// Fetch a Virtual Terminal on your integration
    ///
    /// Takes in the following:
    ///     - `code`: Code of the Virtual Terminal
    pub async fn fetch_virtual_terminal(
        self,
        code: String,
    ) -> PaystackResult<VirtualTerminalResponseData> {
        let url = format!("{}/{}", self.base_url, code);

        let response = self.http.get(&url, &self.key, None).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<VirtualTerminalResponseData> =
                    serde_json::from_str(&response)
                        .map_err(|e| PaystackAPIError::VirtualTerminal(e.to_string()))?;

                Ok(parsed_response)
            }
            Err(e) => Err(PaystackAPIError::VirtualTerminal(e.to_string())),
        }
    }

    /// Update a Virtual Terminal on your integration
    ///
    /// Takes in the following:
    ///     - `code`: Code of the Virtual Terminal to update.
    ///     - `name`: Name of the Virtual Terminal.
    pub async fn update_virtual_terminal(
        &self,
        code: String,
        name: String,
    ) -> PaystackResult<PhantomData<String>> {
        let url = format!("{}/{}", self.base_url, code);
        let body = json!({
            "name": name
        });

        let response = self.http.put(&url, &self.key, &body).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<PhantomData<String>> =
                    serde_json::from_str(&response)
                        .map_err(|e| PaystackAPIError::VirtualTerminal(e.to_string()))?;

                Ok(parsed_response)
            }
            Err(e) => Err(PaystackAPIError::VirtualTerminal(e.to_string())),
        }
    }

    /// Deactivate a Virtual Terminal on your integration
    ///
    /// Takes in the following:
    ///     - `code`: Code of the Virtual Terminal to deactivate.
    pub async fn deactivate_virtual_terminal(
        &self,
        code: String,
    ) -> PaystackResult<PhantomData<String>> {
        let url = format!("{}/{}/deactivate", self.base_url, code);
        let body = json!({}); // empty body cause the route takes none

        let response = self.http.put(&url, &self.key, &body).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<PhantomData<String>> =
                    serde_json::from_str(&response)
                        .map_err(|e| PaystackAPIError::VirtualTerminal(e.to_string()))?;

                Ok(parsed_response)
            }
            Err(e) => Err(PaystackAPIError::VirtualTerminal(e.to_string())),
        }
    }

    /// Add a destination (WhatsApp number) to a Virtual Terminal on your integration
    ///
    /// Takes in the following:
    ///     - `code`: Code of the Virtual Terminal
    ///     - `destinations`: A vector of `DestinationRequest` containing the notification recipients for payments to the Virtual Terminal.
    pub async fn assign_virtual_terminal_destination(
        &self,
        code: String,
        destinations: Vec<DestinationRequest>,
    ) -> PaystackResult<Vec<DestinationResponse>> {
        let url = format!("{}/{}/destination/assign", self.base_url, code);
        let body = json!({
            "destinations": destinations
        });

        let response = self.http.post(&url, &self.key, &body).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<Vec<DestinationResponse>> =
                    serde_json::from_str(&response)
                        .map_err(|e| PaystackAPIError::VirtualTerminal(e.to_string()))?;

                Ok(parsed_response)
            }
            Err(e) => Err(PaystackAPIError::VirtualTerminal(e.to_string())),
        }
    }

    /// Unassign a destination (WhatsApp Number) summary of transactions from a Virtual Terminal on your integration
    ///
    /// Takes in the following:
    ///     - `code`: Code of the Virtual Terminal.
    ///     - `targets`: A vector of destination targets to unassign.
    pub async fn unassign_virtual_terminal_destination(
        &self,
        code: String,
        targets: Vec<String>,
    ) -> PaystackResult<PhantomData<String>> {
        let url = format!("{}/{}/destination/unassign", self.base_url, code);
        let body = json!({
            "targets": targets
        });

        let response = self.http.post(&url, &self.key, &body).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<PhantomData<String>> =
                    serde_json::from_str(&response)
                        .map_err(|e| PaystackAPIError::VirtualTerminal(e.to_string()))?;

                Ok(parsed_response)
            }
            Err(e) => Err(PaystackAPIError::VirtualTerminal(e.to_string())),
        }
    }

    /// Add a split code to a Virtual Terminal on your integration
    ///
    /// Takes in the following:
    ///     - `code`: Code of the Virtual Terminal
    ///     - `split_code`: Split code to be added to the Virtual Terminal
    pub async fn add_split_code_to_virtual_terminal(
        &self,
        code: String,
        split_code: String,
    ) -> PaystackResult<TransactionSplitResponseData> {
        let url = format!("{}/{}/split_code", self.base_url, code);
        let body = json!({
            "split_code": split_code
        });

        let response = self.http.put(&url, &self.key, &body).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<TransactionSplitResponseData> =
                    serde_json::from_str(&response)
                        .map_err(|e| PaystackAPIError::VirtualTerminal(e.to_string()))?;

                Ok(parsed_response)
            }
            Err(e) => Err(PaystackAPIError::VirtualTerminal(e.to_string())),
        }
    }

    /// Remove a split code from a Virtual Terminal on your integration
    ///
    /// Takes in the following:
    ///     - `code`: Code of the Virtual Terminal
    ///     - `split_code`: Split code to be removed from the Virtual Terminal
    pub async fn remove_split_code_from_virtual_terminal(
        &self,
        code: String,
        split_code: String,
    ) -> PaystackResult<PhantomData<String>> {
        let url = format!("{}/{}/split_code", self.base_url, code);
        let body = json!({
            "split_code": split_code
        });

        let response = self.http.delete(&url, &self.key, &body).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<PhantomData<String>> =
                    serde_json::from_str(&response)
                        .map_err(|e| PaystackAPIError::VirtualTerminal(e.to_string()))?;

                Ok(parsed_response)
            }
            Err(e) => Err(PaystackAPIError::VirtualTerminal(e.to_string())),
        }
    }
}
