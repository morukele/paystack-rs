//! Terminal Models
//! ====================
//! This file contains the models and enums for working with the terminal endpoint.

use serde::{Deserialize, Serialize};
use std::fmt;

// This struct is used to create an event body for sending an event to the paystack Terminal using the Paystack API.
/// This struct should be created using the `SendEventBodyBuilder`
/// The Builder derivation allows for the automatic implementation of the builder
#[derive(Deserialize, Serialize)]
pub struct SendEventBody {
    event_type: EventType,
    action: ActionType,
    data: EventData,
}

///
#[derive(Deserialize, Serialize)]
pub struct EventData {
    id: String,
    reference: Option<String>,
}

///
#[derive(Deserialize, Serialize)]
pub enum ActionType {
    ///
    Process,
    ///
    View,
    ///
    Print,
}

impl fmt::Display for ActionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let action = match self {
            ActionType::Process => "process",
            ActionType::View => "view",
            ActionType::Print => "print",
        };
        write!(f, "{}", action)
    }
}

/// Represents the different terminal event types supported by the Paystack API.
///
/// The `EventType` enum defines the possible even types that can be sent to Paystack Terminal.
/// The paystack API currently supports `Invoice` and `Transaction`. This list will be periodically updated as the API evolves. Feel free to open a PR if you catch the change before us.
///
/// # Variants
///
/// - `Invoice`: Invoice event.
/// - `Transaction`: Transaction event.
///
/// # Examples
///
/// ```
/// use paystack::EventType;
///
/// let invoice_event = EventType::Invoice;
/// let terminal_event = EventType::Transaction;
///
/// println!("{:?}", invoice_event); // Prints: invoice
/// ```
/// The example demonstrates the usage of the `EventType` enum from the Paystack crate, creating instances of each variant and printing a debug representation.
///
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[non_exhaustive]
pub enum EventType {
    /// Invoice event
    #[default]
    Invoice,
    /// Transaction event
    Transaction,
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let terminal_type = match self {
            EventType::Invoice => "invoice",
            EventType::Transaction => "transaction",
        };
        write!(f, "{}", terminal_type)
    }
}
