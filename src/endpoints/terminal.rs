//! Terminal
//! ========
//! The Terminal API allows you to build delightful in-person payment experiences.

use std::{marker::PhantomData, sync::Arc};

use crate::{
    EventRequest, FetchEventStatusResponseData, FetchTerminalStatusResponseData, HttpClient,
    PaystackAPIError, PaystackResult, Response, SendEventResponseData, TerminalData,
    UpdateTerminalRequest,
};

/// A struct to hold all the functions of the terminal API endpoint
#[derive(Debug, Clone)]
pub struct TerminalEndpoints<T: HttpClient + Default> {
    /// Paystack API Key
    key: String,
    /// Base URL for the transaction route
    base_url: String,
    /// Http client for the route
    http: Arc<T>,
}

impl<T: HttpClient + Default> TerminalEndpoints<T> {
    /// Constructor
    pub fn new(key: Arc<String>, http: Arc<T>) -> TerminalEndpoints<T> {
        let base_url = String::from("https://api.paystack.co/terminal");
        TerminalEndpoints {
            key: key.to_string(),
            base_url,
            http,
        }
    }

    /// Send an event from your application to the Paystack Terminal
    ///
    /// Takes in the following:
    ///     - `terminal_id`: The ID of the Terminal the event should be sent to.
    ///     - `EventRequest`: A struct containing the information of the event to send to the terminal. It is created with the `EventRequestBuilder`.
    pub async fn send_event(
        &self,
        terminal_id: String,
        event_request: EventRequest,
    ) -> PaystackResult<SendEventResponseData> {
        let url = format!("{}/{}/event", self.base_url, terminal_id);
        let body = serde_json::to_value(event_request)
            .map_err(|e| PaystackAPIError::Terminal(e.to_string()))?;

        let response = self.http.post(&url, &self.key, &body).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<SendEventResponseData> =
                    serde_json::from_str(&response)
                        .map_err(|e| PaystackAPIError::Terminal(e.to_string()))?;

                Ok(parsed_response)
            }
            Err(e) => Err(PaystackAPIError::Terminal(e.to_string())),
        }
    }

    /// Check the status of an event sent to the Paystack Terminal
    ///
    /// Takes in the following:
    ///     - `terminal_id`: The ID of the Terminal the event was sent to.
    ///     - `event_id`: The ID of the event that was sent to the Terminal.
    pub async fn fetch_event_status(
        &self,
        terminal_id: String,
        event_id: String,
    ) -> PaystackResult<FetchEventStatusResponseData> {
        let url = format!("{}/{}/event/{}", self.base_url, terminal_id, event_id);

        let response = self.http.get(&url, &self.key, None).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<FetchEventStatusResponseData> =
                    serde_json::from_str(&response)
                        .map_err(|e| PaystackAPIError::Terminal(e.to_string()))?;

                Ok(parsed_response)
            }
            Err(e) => Err(PaystackAPIError::Terminal(e.to_string())),
        }
    }

    /// Check the availiability of a Terminal before sending an event to it
    ///
    /// Takes in the following:
    ///     - `terminal_id`: The ID of the Terminal you want to check.
    pub async fn fetch_terminal_status(
        &self,
        terminal_id: String,
    ) -> PaystackResult<FetchTerminalStatusResponseData> {
        let url = format!("{}/{}/presence", self.base_url, terminal_id);

        let response = self.http.get(&url, &self.key, None).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<FetchTerminalStatusResponseData> =
                    serde_json::from_str(&response)
                        .map_err(|e| PaystackAPIError::Terminal(e.to_string()))?;

                Ok(parsed_response)
            }
            Err(e) => Err(PaystackAPIError::Terminal(e.to_string())),
        }
    }

    /// List the Terminals available on your integration
    ///
    /// Takes in the following:
    ///     - `per_page`: Specify how many records you want retrieved. Defaults to a value of 50
    pub async fn list_terminals(&self, per_page: Option<i32>) -> PaystackResult<Vec<TerminalData>> {
        let url = format!("{}", self.base_url);
        let per_page = per_page.unwrap_or(50).to_string();
        let query = vec![("perPage", per_page.as_str())];

        let response = self.http.get(&url, &self.key, Some(&query)).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<Vec<TerminalData>> = serde_json::from_str(&response)
                    .map_err(|e| PaystackAPIError::Terminal(e.to_string()))?;

                Ok(parsed_response)
            }
            Err(e) => Err(PaystackAPIError::Terminal(e.to_string())),
        }
    }

    /// Get the details of a Terminal
    ///
    /// Takes in the following:
    ///     - `terminal_id`: The ID of the Terminal the event was sent to.
    pub async fn fetch_terminal(&self, terminal_id: String) -> PaystackResult<TerminalData> {
        let url = format!("{}/{}", self.base_url, terminal_id);

        let response = self.http.get(&url, &self.key, None).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<TerminalData> = serde_json::from_str(&response)
                    .map_err(|e| PaystackAPIError::Terminal(e.to_string()))?;

                Ok(parsed_response)
            }
            Err(e) => Err(PaystackAPIError::Terminal(e.to_string())),
        }
    }

    /// Update the details of a Terminal.
    ///
    /// Takes in the following:
    ///     - `terminal_id`: The ID of the Terminal you want to update.
    ///     - `UpdateTerminalRequest`: Contains the datails to be updated in the terminal. Should be constructed using the `UpdateTerminalRequestBuilder`
    ///
    /// NB: The generic for the result here is a `String`, because there is no data field in the response from the API.
    /// The string will be ignored because the underlying `data` field in the `response` is an `Option`.
    pub async fn update_terminal(
        &self,
        terminal_id: String,
        update_request: UpdateTerminalRequest,
    ) -> PaystackResult<PhantomData<String>> {
        let url = format!("{}/{}", self.base_url, terminal_id);
        let body = serde_json::to_value(update_request)
            .map_err(|e| PaystackAPIError::Terminal(e.to_string()))?;

        let response = self.http.post(&url, &self.key, &body).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<PhantomData<String>> =
                    serde_json::from_str(&response)
                        .map_err(|e| PaystackAPIError::Terminal(e.to_string()))?;

                Ok(parsed_response)
            }
            Err(e) => Err(PaystackAPIError::Terminal(e.to_string())),
        }
    }

    /// Activate your debug device by linking it to your integration
    ///
    /// Take in the following:
    ///     - `serial_number`: The device serial number
    /// NB: The generic for the result here is a `String`, because there is no data field in the response from the API.
    /// The string will be ignored because the underlying `data` field in the `response` is an `Option`.
    pub async fn commission_terminal(
        &self,
        serial_number: String,
    ) -> PaystackResult<PhantomData<String>> {
        let url = format!("{}/commission_device", self.base_url);
        let body = serde_json::json!({
            "serial_number": serial_number
        });

        let response = self.http.post(&url, &self.key, &body).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<PhantomData<String>> =
                    serde_json::from_str(&response)
                        .map_err(|e| PaystackAPIError::Terminal(e.to_string()))?;

                Ok(parsed_response)
            }
            Err(e) => Err(PaystackAPIError::Terminal(e.to_string())),
        }
    }

    /// Unlink your debug device from your integration
    ///
    /// Take in the following:
    ///     - `serial_number`: The device serial number
    /// NB: The generic for the result here is a `String`, because there is no data field in the response from the API.
    /// The string will be ignored because the underlying `data` field in the `response` is an `Option`.
    pub async fn decommission_terminal(
        &self,
        serial_number: String,
    ) -> PaystackResult<PhantomData<String>> {
        let url = format!("{}/decommission_device", self.base_url);
        let body = serde_json::json!({
            "serial_number": serial_number
        });

        let response = self.http.post(&url, &self.key, &body).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<PhantomData<String>> =
                    serde_json::from_str(&response)
                        .map_err(|e| PaystackAPIError::Terminal(e.to_string()))?;

                Ok(parsed_response)
            }
            Err(e) => Err(PaystackAPIError::Terminal(e.to_string())),
        }
    }
}
