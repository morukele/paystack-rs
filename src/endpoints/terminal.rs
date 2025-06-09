//! Terminal
//! ========
//! The Terminal API allows you to build delightful in-person payment experiences.

use std::sync::Arc;

use crate::{EventRequest, HttpClient, PaystackResult, SendEventResponseData};

/// A struct to hold all the functions of the terminal API endpoint
#[derive(Debug, Clone)]
pub struct TerminalEndpoints<T: HttpClient + Default> {
    key: String,
    base_url: String,
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
    pub fn send_event(
        &self,
        terminal_id: String,
        event_request: EventRequest,
    ) -> PaystackResult<SendEventResponseData> {
        todo!()
    }
}
