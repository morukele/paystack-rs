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
///     - currency (Optional): Currency in which amount should be charged (NGN, GHS, ZAR or USD). Defaults to your integration currency.
#[derive(serde::Serialize, Debug)]
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

/// This struct is used to create a charge body for creating a Charge Authorization using the Paystack API.
///
/// IMPORTANT: This class can only be created using the ChargeBuilder.
///
/// The struct has the following fields:
///     - amount: Amount should be in the smallest unit of the currency e.g. kobo if in NGN and cents if in USD
///     - email: Customer's email address
///     - currency (Optional): Currency in which amount should be charged (NGN, GHS, ZAR or USD). Defaults to your integration currency.
///     - authorizatuin_code: A valid authorization code to charge
///     - reference (Optional): Unique transaction reference. Only -, ., = and alphanumeric characters allowed.
///     - channel (Optional): Send us 'card' or 'bank' or 'card','bank' as an array to specify what options to show the user paying
///     - transaction_charge (Optional): A flat fee to charge the subaccount for this transaction
///     (in kobo if currency is NGN, pesewas, if currency is GHS, and cents, if currency is ZAR).
///     This overrides the split percentage set when the subaccount was created.
///     Ideally, you will need to use this if you are splitting in flat rates
///     (since subaccount creation only allows for percentage split). e.g. 7000 for a 70 naira

#[derive(serde::Serialize, Debug)]
pub struct Charge {
    email: String,
    amount: String,
    authorization_code: String,
    reference: Option<String>,
    currency: Option<String>,
    channel: Option<Vec<String>>,
    transaction_charge: Option<u32>,
}

/// Builder for the Charge object
#[derive(Default, Clone)]
pub struct ChargeBuilder {
    email: Option<String>,
    amount: Option<String>,
    authorization_code: Option<String>,
    reference: Option<String>,
    currency: Option<String>,
    channel: Option<Vec<String>>,
    transaction_charge: Option<u32>,
}

impl ChargeBuilder {
    /// Create a new instance of the Transaction builder with default properties
    pub fn new() -> Self {
        ChargeBuilder::default()
    }

    /// Specify the transaction email
    pub fn email(mut self, email: impl Into<String>) -> Self {
        self.email = Some(email.into());
        self
    }

    /// Specify the Transaction amount
    pub fn amount(mut self, amount: impl Into<String>) -> Self {
        self.amount = Some(amount.into());
        self
    }

    /// Specify the charge Authorization Code
    pub fn authorization_code(mut self, code: impl Into<String>) -> Self {
        self.authorization_code = Some(code.into());
        self
    }

    /// Specify charge reference
    pub fn reference(mut self, reference: impl Into<String>) -> Self {
        self.reference = Some(reference.into());
        self
    }

    /// Specify charge currency
    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = Some(currency.into());
        self
    }

    /// Specify charge channel
    pub fn channel(mut self, channel: Vec<String>) -> Self {
        self.channel = Some(channel);
        self
    }

    /// Specify charge transaction charge
    pub fn transaction_charge(mut self, transaction_charge: u32) -> Self {
        self.transaction_charge = Some(transaction_charge);
        self
    }

    /// Build the Charge object
    pub fn build(self) -> PaystackResult<Charge> {
        let Some(email) = self.email else {
            return Err(error::PaystackError::Charge("email is required for creating a charge".to_string()))
        };

        let Some(amount) = self.amount else {
            return Err(error::PaystackError::Charge(
                "amount is required for creating charge".to_string()
            ))
        };

        let Some(authorization_code) = self.authorization_code else {
            return Err(error::PaystackError::Charge(
                "authorization code is required for creating a charge".to_string()
            ))
        };

        Ok(Charge {
            email,
            amount,
            authorization_code,
            reference: self.reference,
            currency: self.currency,
            channel: self.channel,
            transaction_charge: self.transaction_charge,
        })
    }
}
