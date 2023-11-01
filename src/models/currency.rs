//! Currency
//! ===============
//! This file contains the currency options for the paystack API.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents different currencies supported by the Paystack API.
///
/// The `Currency` enum defines the possible currency options that can be used with Paystack,
/// including Nigerian Naira (NGN), Ghanaian Cedis (GHS), American Dollar (USD),
/// and South African Rands (ZAR). It also includes an `EMPTY` variant to represent cases
/// where the currency can be empty.
///
/// # Variants
///
/// - `NGN`: Nigerian Naira.
/// - `GHS`: Ghanaian Cedis.
/// - `USD`: American Dollar.
/// - `ZAR`: South African Rands.
/// - `EMPTY`: Used when the currency can be empty.
///
/// # Examples
///
/// ```
/// use paystack::Currency;
///
/// let ngn_currency = Currency::NGN;
/// let ghs_currency = Currency::GHS;
/// let usd_currency = Currency::USD;
/// let zar_currency = Currency::ZAR;
/// let empty_currency = Currency::EMPTY;
///
/// println!("{:?}", ngn_currency); // Prints: NGN
/// ```
///
/// The example demonstrates the usage of the `Currency` enum from the Paystack crate,
/// creating instances of each variant and printing a debug representation.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum Currency {
    /// Nigerian Naira
    #[default]
    NGN,
    /// Ghanaian Cedis
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
