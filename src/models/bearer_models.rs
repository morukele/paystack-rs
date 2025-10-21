//! Bearer Type
//! =================
//! This file contains the charge bearer option for the paystack API.

use serde::Serialize;
use std::fmt;

/// Represents the type of bearer for a charge.
///
/// The `BearerType` enum defines the possible types of bearers for a charge, indicating who
/// is responsible for the transaction split.
///
/// # Variants
///
/// - `Subaccount`: The subaccount bears the transaction split.
/// - `Account`: The main account bears the transaction split.
/// - `AllProportional`: The transaction is split proportionally to all accounts.
/// - `All`: The transaction is paid by all accounts.
///
/// # Examples
///
/// ```
/// use paystack::BearerType;
///
/// let subaccount_bearer = BearerType::Subaccount;
/// let account_bearer = BearerType::Account;
/// let all_proportional_bearer = BearerType::AllProportional;
/// let all_bearer = BearerType::All;
///
/// println!("{:?}", subaccount_bearer); // Prints: Subaccount
/// ```
///
/// The example demonstrates the usage of the `BearerType` enum, creating instances of each variant
/// and printing their debug representation.
#[derive(Debug, Serialize, Clone, Default)]
#[serde(rename_all = "lowercase")]
pub enum BearerType {
    /// The subaccount bears the transaction split
    #[default]
    Subaccount,
    /// The main account bears the transaction split
    Account,
    /// The transaction is split proportionally to all accounts
    #[serde(rename = "all-proportional")]
    AllProportional,
    /// The transaction is paid by all accounts
    All,
}

impl fmt::Display for BearerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lowercase_string = match self {
            BearerType::Subaccount => "subaccount",
            BearerType::Account => "account",
            BearerType::AllProportional => "all-proportional",
            BearerType::All => "all",
        };
        write!(f, "{}", lowercase_string)
    }
}
