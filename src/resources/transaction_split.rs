//! Transaction Split
//! =================
//! This file contains the structs and definitions need to create
//! transaction splits for the Paystack API.

use crate::{error::PaystackError, BearerType, Currency, PaystackResult, SplitType};
use serde::Serialize;

/// This struct is used to create a split payment on your integration.
///
/// IMPORTANT: This class can obly be created using the TransactionSplitBuilder.
/// THe struct has the following fields:
///     - name: Name of the transaction split
///     - type: The type of transaction split you want to create.
///       You can use one of the following: percentage | flat.
///     - currency: Any of NGN, GHS, ZAR, or USD
///     - subaccounts: A list of object containing subaccount
///       code and number of shares
///     - bearer_type: Any of subaccount | account | all-proportional | all
///     - bearer_subaccount: Subaccount code
#[derive(Serialize, Debug, Default)]
pub struct TransactionSplit {
    name: String,
    #[serde(rename = "type")]
    split_type: SplitType,
    currency: Currency,
    subaccounts: Vec<Subaccount>,
    bearer_type: BearerType,
    bearer_subaccount: String,
}

/// This struct represents the subacount that bears the transaction split
#[derive(Serialize, Debug, Clone)]
pub struct Subaccount {
    /// This is the sub account code
    pub subaccount: String,
    /// This is the transaction share for the subaccount
    pub share: u32,
}

impl Subaccount {
    /// Creates a new subaccount for the Paystack API
    pub fn new(subaccount: impl Into<String>, share: u32) -> Self {
        Subaccount {
            subaccount: subaccount.into(),
            share,
        }
    }
}

/// A builder pattern implementation for constructing `PercentageSplit` instances.
///
/// The `PercentageSplitBuilder` struct provides a fluent and readable way to construct
/// instances of the `PercentageSplit` struct.
///
/// # Errors
///
/// Returns a `PaystackResult` with an `Err` variant if any required fields are missing,
/// including email, amount, and currency. The error indicates which field is missing.
///
/// # Examples
///
/// ```rust
/// use paystack::{Currency, SplitType, BearerType, PercentageSplitBuilder, Subaccount};
///
/// let sub_accounts = vec![
///     Subaccount::new(
///         "ACCT_z3x6z3nbo14xsil",
///         20,
///     ),
///     Subaccount::new(
///         "ACCT_pwwualwty4nhq9d",
///         30,
///     ),
/// ];
///
/// let percentage_split = PercentageSplitBuilder::new()
///     .name("Percentage Split")
///     .split_type(SplitType::Percentage)
///     .currency(Currency::NGN)
///     .add_subaccount(sub_accounts)
///     .bearer_type(BearerType::Subaccount)
///     .bearer_subaccount("ACCT_hdl8abxl8drhrl3")
///     .build();
/// ```
#[derive(Debug, Default, Clone)]
pub struct PercentageSplitBuilder {
    name: Option<String>,
    split_type: Option<SplitType>,
    currency: Option<Currency>,
    subaccounts: Option<Vec<Subaccount>>,
    bearer_type: Option<BearerType>,
    bearer_subaccount: Option<String>,
}

impl PercentageSplitBuilder {
    /// Create a new instance of the Percentage Split builder with default properties
    pub fn new() -> Self {
        PercentageSplitBuilder::default()
    }

    /// Specify the transaction split name
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Specify the transaction split amount
    pub fn split_type(mut self, split_type: SplitType) -> Self {
        self.split_type = Some(split_type);
        self
    }

    /// Specify the transaction split currency
    pub fn currency(mut self, currency: Currency) -> Self {
        self.currency = Some(currency);
        self
    }

    /// Specify the subaccount for the transaction split
    pub fn add_subaccount(mut self, sub_accounts: Vec<Subaccount>) -> Self {
        self.subaccounts = Some(sub_accounts);
        self
    }

    /// Specify the bearer type for the transaction split
    pub fn bearer_type(mut self, bearer_type: BearerType) -> Self {
        self.bearer_type = Some(bearer_type);
        self
    }

    /// Specify the bearer subaccount code
    pub fn bearer_subaccount(mut self, code: impl Into<String>) -> Self {
        self.bearer_subaccount = Some(code.into());
        self
    }

    /// Build the TransactionSplit object
    pub fn build(self) -> PaystackResult<TransactionSplit> {
        let Some(name) = self.name else {
            return Err(
                PaystackError::TransactionSplit("name is required to create a transaction split".to_string())
            )
        };

        let Some(split_type) = self.split_type else {
            return Err(
                PaystackError::TransactionSplit("split type is required to create a transaction split".to_string())
            )
        };

        let Some(currency) = self.currency else {
            return Err(
                PaystackError::Transaction(
                    "currency is required to create a transaction split".to_string()
                )
            )
        };

        let Some(subaccounts) = self.subaccounts else {
            return Err(
                PaystackError::TransactionSplit(
                    "sub accounts are required to create a transaction split".to_string()
                )
            )
        };

        let Some(bearer_type) = self.bearer_type else {
            return Err(
                PaystackError::TransactionSplit(
                    "bearer type is required to create a transaction split".to_string()
                )
            )
        };

        let Some(bearer_subaccount) = self.bearer_subaccount else {
            return Err(
                PaystackError::TransactionSplit(
                    "bearer subaccount is required to create a transaction split".to_string()
                )
            )
        };

        Ok(TransactionSplit {
            name,
            split_type,
            currency,
            subaccounts,
            bearer_type,
            bearer_subaccount,
        })
    }
}
