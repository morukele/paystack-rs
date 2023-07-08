//! Paystack Enums
//! ===============
//! This file contains enums of options sent to or returned from the Paystack API

use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
/// Respresents the currencies supported by Paystack
pub enum Currency {
    /// Nigerian Naira
    NGN,
    /// Ghanian Cedis
    GHS,
    /// American Dollar
    USD,
    /// South African Rands
    ZAR,
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
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
/// Represents the status of the Transaction.
pub enum Status {
    /// A successful transaction.
    Success,
    /// An abadoned transaction.
    Abadoned,
    /// A failed transaction
    Failed,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}
