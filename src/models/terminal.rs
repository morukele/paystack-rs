//! Terminal Models
//! ====================
//! This file contains the models and enums for working with the terminal endpoint.

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use std::fmt;

/// This struct is used to create an event body for sending an event to the paystack Terminal using the Paystack API.
/// This struct should be created using the `SendEventBodyBuilder`
/// The Builder derivation allows for the automatic implementation of the builder
#[derive(Serialize, Builder, Debug, Default, Clone)]
pub struct SendEventBody {
    /// The type of event to push. Currently support `invoice` and `transaction`
    event_type: EventType,
    /// The action the Terminal needs to perform.
    action: ActionType,
    /// The parameters needed to perform the specified action.
    data: EventData,
}

/// The data about the event to send to the API.
/// For the `invoice` type, you need to pass the invoice id and offline reference: `{id: invoice_id, reference: offline_reference}`.
/// For the `transaction type, you can pass the transaction id: `{id: transaction_id}`
#[derive(Deserialize, Serialize, Clone, Debug, Default)]
pub struct EventData {
    /// Transaction id
    pub id: String,
    /// Transaction offline reference. Only required for `invoice` type.
    pub reference: Option<String>,
}

/// This struct represents the response of sending an event to the terminal.
#[derive(Deserialize, Debug, Clone)]
pub struct SendEventResponse {
    /// This lets you know if your response was successful or not.
    pub status: bool,
    /// This is a summary of the response and its status.
    pub message: String,
    /// This contains the results of your request.
    pub data: SendEventResponseData,
}

/// This struct represents the data of the event response.
#[derive(Deserialize, Debug, Clone)]
pub struct SendEventResponseData {
    /// Id of the sent event.
    pub id: String,
}

/// This struct represents the response for checking the status of an event sent to the Terminal
#[derive(Deserialize, Debug, Clone)]
pub struct FetchEventStatusResponse {
    /// This lets you know if your response was successful or not.
    pub status: bool,
    /// This is a summary of the response and its status.
    pub message: String,
    /// This contains the results of your request.
    pub data: FetchEventStatusResponseData,
}

/// This struct represents the data of the event status
pub struct FetchEventStatusResponseData {
    /// If the event has been delivered or not.
    pub delivered: bool,
}

/// This struct represents the response for checking the status of an event sent to the Terminal
#[derive(Deserialize, Debug, Clone)]
pub struct FetchTerminalStatusResponse {
    /// This lets you know if your response was successful or not.
    pub status: bool,
    /// This is a summary of the response and its status.
    pub message: String,
    /// This contains the results of your request.
    pub data: FetchTerminalStatusResponseData,
}

/// This struct represents the data of the event status
#[derive(Deserialize, Debug, Clone)]
pub struct FetchTerminalStatusResponseData {
    /// If the event has been delivered or not.
    pub online: bool,
    pub available: bool,
}

/// Represents the different action the Terminal needs to perform.
///
/// # Variants
///
/// - `Process`: Process action.
/// - `View`: View action.
/// - `Print`: Print action.
///
/// # Examples
///
/// ```
/// use paystack::ActionType;
///
/// let process_action = ActionType::Process;
/// let print_action = ActionType::Print;
/// let view_action = ActionType::View;
///
/// println!("{:?}", process_action); // Prints: process
/// ```
#[derive(Serialize, Debug, Clone, Default)]
pub enum ActionType {
    /// Process the event. Valid for all event types.
    #[default]
    Process,
    /// View the event. Valid for only Invoice event type.
    View,
    /// Print the event. Valid for only Transaction event type.
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
#[derive(Debug, Serialize, Clone, Default)]
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
