//! Terminal
//! ==========
//! This file contains the models and options for the Terminal endpoint of the Paystack

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// The request body to send an event from your application to the Paystack Terminal
#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
pub struct EventRequest {
    #[serde(rename = "type")]
    pub event_type: EventType,
    pub action: TerminalAction,
    pub data: EventRequestData,
}

/// The paramters needed to perform the specified action.
///
/// For the invoice type, you need to pass the invoice id and offline reference: {id: invoice_id, reference: offline_reference}.
///
/// For the transaction type, you can pass the transaction id: {id: transaction_id}, reference field can be `None`
#[derive(Debug, Serialize, Deserialize, Clone, Builder)]
pub struct EventRequestData {
    pub id: String,
    #[builder(setter(strip_option), default)]
    pub reference: Option<String>,
}

/// The type of event to push.
/// Paystack currently support `invoice` and `transaction`
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum EventType {
    Invoice,
    Transaction,
}

/// The action the Terminal needs to perform.
///
/// - For the `Invoice` type, the action can either be `Process` or `View`.
///
/// - For the `Transaction` type, the action can either be `Process` or `Print`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum TerminalAction {
    Process,
    View,
    Print,
}

impl fmt::Display for TerminalAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let action = match self {
            TerminalAction::Process => "process",
            TerminalAction::Print => "print",
            TerminalAction::View => "view",
        };
        write!(f, "{}", action)
    }
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let event = match self {
            EventType::Invoice => "invoice",
            EventType::Transaction => "transaction",
        };
        write!(f, "{}", event)
    }
}

/// Update request for terminal
#[derive(Debug, Serialize, Deserialize, Builder, Default)]
pub struct UpdateTerminalRequest {
    /// Name of the terminal
    #[builder(setter(strip_option), default)]
    pub address: Option<String>,
    /// The address of the terminal
    #[builder(setter(strip_option), default)]
    pub name: Option<String>,
}

/// Response data for the send event route in the terminal endpoint.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct SendEventResponseData {
    pub id: String,
}

/// Response data for the fetch event status route in the terminal endpoint.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FetchEventStatusResponseData {
    pub delivered: bool,
}

/// Response data for fetch terminal status route in the terminal endpoint.
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct FetchTerminalStatusResponseData {
    pub online: bool,
    pub available: bool,
}

/// Response data for terminal
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct TerminalData {
    pub id: u64,
    pub serial_number: String,
    pub device_make: Option<String>,
    pub terminal_id: String,
    pub integration: u64,
    pub domain: String,
    pub name: String,
    pub address: Option<String>,
    pub status: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_event_request() {
        let even_request_data = EventRequestDataBuilder::default()
            .id("some-id".to_string())
            .reference("some-ref".to_string())
            .build()
            .expect("failed to build event request data");

        let event_request = EventRequestBuilder::default()
            .event_type(EventType::Invoice)
            .action(TerminalAction::Process)
            .data(even_request_data)
            .build()
            .expect("failed to build event request");

        assert_eq!(&event_request.event_type, &EventType::Invoice);
        assert_eq!(&event_request.action, &TerminalAction::Process);
        assert_eq!(&event_request.data.id, "some-id");
        assert_eq!(&event_request.data.reference, &Some("some-ref".to_string()))
    }

    #[test]
    fn create_update_terminal_request() {
        let update_request = UpdateTerminalRequestBuilder::default()
            .address("some-address".to_string())
            .name("some-name".to_string())
            .build()
            .expect("failed to build update terminal request");

        assert_eq!(update_request.address, Some("some-address".to_string()));
        assert_eq!(update_request.name, Some("some-name".to_string()));
    }
}
