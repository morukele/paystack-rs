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
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EventRequestData {
    pub id: String,
    pub reference: Option<String>,
}

/// The type of event to push.
/// Paystack currently support `invoice` and `transaction`
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum EventType {
    Invoice,
    Transaction,
}

/// The action the Terminal needs to perform.
///
/// - For the `Invoice` type, the action can either be `Process` or `View`.
///
/// - For the `Transaction` type, the action can either be `Process` or `Print`.
#[derive(Debug, Serialize, Deserialize, Clone)]
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

/// Contains response data for the send event route in the terminal endpoint.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SendEventResponseData {
    pub id: String,
}
