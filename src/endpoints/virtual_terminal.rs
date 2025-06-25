//! Virtual Terminal
//! ================
//! The Virtual Terminal API allows you to accept in-person payments without a POS device.

use std::sync::Arc;

use crate::{
    HttpClient, PaystackAPIError, PaystackResult, Response, VirtualTerminalRequestData,
    VirtualTerminalResponseData,
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
}
