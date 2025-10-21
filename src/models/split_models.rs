//! Split Type
//! ===============
//! This file contains the transaction split options for the paystack API.

use serde::Serialize;
use std::fmt;

/// Represents the type of transaction split.
///
/// The `SplitType` enum defines the possible types of transaction splits that can be created,
/// indicating whether the split is based on a percentage or a flat amount.
///
/// # Variants
///
/// - `Percentage`: A split based on a percentage.
/// - `Flat`: A split based on an amount.
///
/// # Examples
///
/// ```
/// use paystack::SplitType;
///
/// let percentage_split = SplitType::Percentage;
/// let flat_split = SplitType::Flat;
///
/// println!("{:?}", percentage_split); // Prints: Percentage
/// ```
///
/// The example demonstrates the usage of the `SplitType` enum, creating instances of each variant
/// and printing their debug representation.
#[derive(Debug, Serialize, Clone, Default)]
#[serde(rename_all = "lowercase")]
pub enum SplitType {
    /// A split based on a percentage
    #[default]
    Percentage,
    /// A split based on an amount
    Flat,
}

impl fmt::Display for SplitType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lowercase_string = match self {
            SplitType::Percentage => "percentage",
            SplitType::Flat => "flat",
        };
        write!(f, "{lowercase_string}")
    }
}
