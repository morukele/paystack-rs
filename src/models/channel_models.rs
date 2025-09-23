//! Channel
//! ===============
//! This file contains the Channel option for the paystack API.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents the payment channels supported by Paystack.
///
/// The `Channel` enum defines the possible payment channels that can be used with Paystack,
/// including debit card, bank interface, USSD code, QR code, mobile money, bank transfer,
/// and Apple Pay.
///
/// # Variants
///
/// - `Card`: Payment with a debit card.
/// - `Bank`: Payment with a bank interface.
/// - `Ussd`: Payment with a USSD code.
/// - `Qr`: Payment with a QR code.
/// - `MobileMoney`: Payment with mobile money.
/// - `BankTransfer`: Payment with a bank transfer.
/// - `ApplePay`: Payment with Apple Pay.
///
/// # Examples
///
/// ```
/// use paystack::Channel;
///
/// let card = Channel::Card;
/// let bank = Channel::Bank;
/// let ussd = Channel::Ussd;
/// let qr = Channel::Qr;
/// let mobile_money = Channel::MobileMoney;
/// let bank_transfer = Channel::BankTransfer;
/// let apple_pay = Channel::ApplePay;
///
/// println!("{:?}", card); // Prints: card
/// println!("{:?}", mobile_money); // Prints: mobile_money
/// ```
///
/// The example demonstrates the usage of the `Channel` enum from the Paystack crate,
/// creating instances of each variant and printing their debug representation.
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Channel {
    /// Debit Card
    #[default]
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
        let lower_case = match self {
            Channel::Card => "card",
            Channel::Bank => "bank",
            Channel::Ussd => "ussd",
            Channel::Qr => "qr",
            Channel::MobileMoney => "mobile_money",
            Channel::BankTransfer => "bank_transfer",
            Channel::ApplePay => "mobile_money",
        };
        write!(f, "{lower_case}")
    }
}
