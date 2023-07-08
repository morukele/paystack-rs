//! Transactions
//! =============
//! This file contains all the structs and definitions needed to
//! create a transaction using the paystack API.

use crate::{error, Currency, PaystackResult};
use serde::Serialize;

/// This struct is used to create a transaction body for creating a transaction using the Paystack API.
///
/// IMPORTANT: This class can only be created using the TransactionBuilder.
///
/// The struct has the following fields:
///     - amount: Amount should be in the smallest unit of the currency e.g. kobo if in NGN and cents if in USD
///     - email: Customer's email address
///     - currency (Optional): Currency in which amount should be charged (NGN, GHS, ZAR or USD). Defaults to your integration currency.
#[derive(Serialize, Debug)]
pub struct Transaction {
    amount: String,
    email: String,
    currency: Option<Currency>,
}

/// Builder for the Transaction object
#[derive(Default, Clone)]
pub struct TransactionBuilder {
    amount: Option<String>,
    email: Option<String>,
    currency: Option<Currency>,
}

impl TransactionBuilder {
    /// Create a new instance of the Transaction builder with default properties
    pub fn new() -> Self {
        TransactionBuilder::default()
    }

    /// Specify the Transaction email
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    /// Specify the Transaction amount
    pub fn amount(mut self, amount: impl Into<String>) -> Self {
        self.amount = Some(amount.into());
        self
    }

    /// Specify the Transaction currency
    pub fn currency(mut self, currency: Currency) -> Self {
        self.currency = Some(currency);
        self
    }

    /// Build the Transaction object
    pub fn build(self) -> PaystackResult<Transaction> {
        let Some(email) = self.email else {
            return Err(error::PaystackError::Transaction("email is required for transaction".to_string()))
        };

        let Some(amount) = self.amount else {
            return Err(error::PaystackError::Transaction("amount is required for transaction".to_string()))
        };

        Ok(Transaction {
            email,
            amount,
            currency: self.currency,
        })
    }
}
