//! Status
//! ===============
//! This file contains the status options for the transactions in the paystack API.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Represents the status of a transaction.
///
/// The `Status` enum defines the possible status values for a transaction,
/// indicating whether the transaction was successful, abandoned, or failed.
///
/// # Variants
///
/// - `Success`: Represents a successful transaction.
/// - `Abandoned`: Represents an abandoned transaction.
/// - `Failed`: Represents a failed transaction.
///
/// # Examples
///
/// ```
/// use paystack::Status;
///
/// let success_status = Status::Success;
/// let abandoned_status = Status::Abandoned;
/// let failed_status = Status::Failed;
///
/// println!("{:?}", success_status); // Prints: Success
/// ```
///
/// The example demonstrates the usage of the `Status` enum, creating instances of each variant
/// and printing their debug representation.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    /// A successful transaction.
    Success,
    /// An abandoned transaction.
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
        write!(f, "{lowercase_string}")
    }
}
