//! Transactions
//! =============
//! This file contains all the structs and definitions needed to
//! use the transaction route of the paystack API

use crate::{error, PaystackResult};

/// This struct is used to create a transaction body for creating a transaction using the Paystack API.
///
/// IMPORTANT: This class can only be created using the TransactionBuilder.
///
/// The struct has the following fields:
///     - amount: Amount should be in the smallest unit of the currency e.g. kobo if in NGN and cents if in USD
///     - email: Customer's email address
///     - currency (Optional): The transaction currency (NGN, GHS, ZAR or USD). Defaults to your integration currency.
#[derive(serde::Serialize)]
pub struct Transaction {
    amount: String,
    email: String,
    currency: Option<String>,
}

/// Builder for the Transaction object
#[derive(Default, Clone)]
pub struct TransactionBuilder {
    amount: Option<String>,
    email: Option<String>,
    currency: Option<String>,
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
    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = Some(currency.into());
        self
    }

    /// Build the Transaction object
    pub fn build(self) -> PaystackResult<Transaction> {
        let Some(email) = self.email else {
            return Err(error::PaystackError::TransactionCreation("email is required for transaction".to_string()))
        };

        let Some(amount) = self.amount else {
            return Err(error::PaystackError::TransactionCreation("amount is required for transaction".to_string()))
        };

        Ok(Transaction {
            email,
            amount,
            currency: self.currency,
        })
    }
}
