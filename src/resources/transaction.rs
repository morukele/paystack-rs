//! Transactions
//! =============
//! This file contains all the structs and definitions needed to
//! create a transaction using the paystack API.

use crate::{error::Error, Channel, Currency, PaystackResult};
use serde::Serialize;

/// This struct is used to create a transaction body for creating a transaction using the Paystack API.
///
/// IMPORTANT: This class can only be created using the TransactionBuilder.
///
/// The struct has the following fields:
///     - amount: Amount should be in the smallest unit of the currency e.g. kobo if in NGN and cents if in USD
///     - email: Customer's email address
///     - currency (Optional): Currency in which amount should be charged (NGN, GHS, ZAR or USD). Defaults to your integration currency.
#[derive(Serialize, Debug, Default)]
pub struct Transaction {
    amount: String,
    email: String,
    currency: Option<Currency>,
    channels: Option<Vec<Channel>>,
}

/// Builder for the Transaction object
#[derive(Default, Clone)]
pub struct TransactionBuilder {
    amount: Option<String>,
    email: Option<String>,
    currency: Option<Currency>,
    channels: Option<Vec<Channel>>,
}

impl TransactionBuilder {
    /// Create a new instance of the Transaction builder with default properties
    pub fn new() -> Self {
        TransactionBuilder::default()
    }

    /// Specify email for the Transaction
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

    /// Specify the Tranaction channels
    pub fn channels(mut self, channels: Vec<Channel>) -> Self {
        self.channels = Some(channels);
        self
    }

    /// Build the Transaction object
    pub fn build(self) -> PaystackResult<Transaction> {
        let Some(email) = self.email else {
            return Err(
                Error::Transaction("email is required for transaction".to_string())
            )
        };

        let Some(amount) = self.amount else {
            return Err(
                Error::Transaction("amount is required for transaction".to_string())
            )
        };

        Ok(Transaction {
            email,
            amount,
            currency: self.currency,
            channels: self.channels,
        })
    }
}

/// This struct is used to create a partial debit transaction body for creating a partial debit using the Paystack API.
///
/// IMPORTANT: This class can only be created using the PartialDebitTransactionBuilder.
///
/// The struct has the following fields:
///     - authorization_code: Authorization Code for the transaction
///     - amount: Amount should be in the smallest unit of the currency e.g. kobo if in NGN and cents if in USD
///     - email: Customer's email address
///     - currency : Currency in which amount should be charged (NGN, GHS, ZAR or USD). Defaults to your integration currency.
///     - reference (Optional): Unique transaction reference.
///     - at_least: Minimum amount to charge
#[derive(Debug, Clone, Serialize, Default)]
pub struct PartialDebitTransaction {
    authorization_code: String,
    amount: String,
    email: String,
    currency: Currency,
    reference: Option<String>,
    at_least: Option<String>,
}

/// Builder for the Transaction object
#[derive(Default, Clone)]
pub struct PartialDebitTransactionBuilder {
    authorization_code: Option<String>,
    amount: Option<String>,
    email: Option<String>,
    currency: Option<Currency>,
    reference: Option<String>,
    at_least: Option<String>,
}

impl PartialDebitTransactionBuilder {
    /// Create new instance of the Partial Debit Transaction builder with default properties.
    pub fn new() -> Self {
        PartialDebitTransactionBuilder::default()
    }

    /// Specify the authorization code.
    pub fn authorization_code(mut self, authorization_code: impl Into<String>) -> Self {
        self.authorization_code = Some(authorization_code.into());
        self
    }

    /// Specify the transaction amount.
    pub fn amount(mut self, amount: impl Into<String>) -> Self {
        self.amount = Some(amount.into());
        self
    }

    /// Specify email for the Transaction.
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    /// Specify transaction currency.
    pub fn currency(mut self, currency: Currency) -> Self {
        self.currency = Some(currency);
        self
    }

    /// Specify the transaction reference.
    pub fn reference(mut self, reference: impl Into<String>) -> Self {
        self.reference = Some(reference.into());
        self
    }

    /// Specify the minimum amount to charge for the transaction.
    pub fn at_least(mut self, at_least: impl Into<String>) -> Self {
        self.at_least = Some(at_least.into());
        self
    }

    /// Build the PartialDebitTransaction object
    pub fn build(self) -> PaystackResult<PartialDebitTransaction> {
        let Some(authorization_code) = self.authorization_code else {
            return Err(
                Error::Transaction("authorization code is required for partial debit transaction".to_string())
            )
        };

        let Some(email) = self.email else {
            return Err(
                Error::Transaction("email is required for transaction".to_string())
            )
        };

        let Some(currency) = self.currency else {
            return Err(
                Error::Transaction("currency is required for transaction".to_string())
            )
        };

        let Some(amount) = self.amount else {
            return Err(
                Error::Transaction("amount is required for transaction".to_string())
            )
        };

        Ok(PartialDebitTransaction {
            authorization_code,
            email,
            amount,
            currency,
            reference: self.reference,
            at_least: self.at_least,
        })
    }
}
