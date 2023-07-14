//! Paystack Enums
//! ===============
//! This file contains enums of options sent to or returned from the Paystack API

use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
/// Respresents the currencies supported by Paystack
pub enum Currency {
    /// Nigerian Naira
    #[default]
    NGN,
    /// Ghanian Cedis
    GHS,
    /// American Dollar
    USD,
    /// South African Rands
    ZAR,
    /// Used when currency can be empty.
    EMPTY,
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let currency = match self {
            Currency::NGN => "NGN",
            Currency::GHS => "GHS",
            Currency::USD => "USD",
            Currency::ZAR => "ZAR",
            Currency::EMPTY => "",
        };
        write!(f, "{}", currency)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
/// Represents the payment channels supported by Paystack
pub enum Channel {
    /// Debit Card
    Card,
    /// Payment with Bank Interface
    Bank,
    /// Payment with USSD Code
    Ussd,
    /// Payment with QR Code
    Qr,
    /// Payment with Mobile Money
    MobileMoney,
    /// Payment with Bank Transfer
    BankTransfer,
    /// Payment with Apple Pay
    ApplePay,
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
/// Represents the status of the Transaction.
pub enum Status {
    /// A successful transaction.
    Success,
    /// An abadoned transaction.
    Abandoned,
    /// A failed transaction.
    Failed,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lowercase_string = match self {
            Status::Success => "success",
            Status::Abandoned => "abandoned",
            Status::Failed => "failed",
        };
        write!(f, "{}", lowercase_string)
    }
}
