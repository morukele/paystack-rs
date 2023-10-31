//! Terminal
//! ========
//! The Terminal API allows you to build delightful in-person payment experiences.

use crate::SendEventBody;

/// A Struct to hold all the functions of the terminal API route
#[derive(Debug, Clone)]
pub struct TerminalEndpoints<'a> {
    api_key: &'a str,
}

static BASE_URL: &str = "https://api.paystack.co";

impl<'a> TerminalEndpoints<'a> {
    /// Constructor for the terminal object
    pub fn new(key: &'a str) -> TerminalEndpoints<'a> {
        TerminalEndpoints { api_key: key }
    }

    /// Send an event from your application to the Paystack Terminal
    pub async fn send_event(terminal_id: &str, event_body: SendEventBody) {}
}
