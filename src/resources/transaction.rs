//! Transactions
//! =============
//! This file contains all the structs and definitions needed to
//! create a transaction using the paystack API.

use crate::{Channel, Currency};
use serde::Serialize;
use derive_builder::Builder;

/// This struct is used to create a transaction body for creating a transaction using the Paystack API.
/// This struct should be created using the `InitializeTransactionBodyBuilder`
/// The Builder derivation allows for the automatic implementation of the builder pattern.
#[derive(Serialize, Debug, Default, Builder)]
pub struct InitializeTransactionBody {
    /// Amount should be in the smallest unit of the currency e.g. kobo if in NGN and cents if in USD
    amount: String,
    /// Customer's email address
    email: String,
    /// Currency in which amount should be charged (NGN, GHS, ZAR or USD). Defaults to your integration currency.
    #[builder(default = "None")]
    currency: Option<Currency>,
    /// Unique transaction reference. Only -, ., = and alphanumeric characters allowed.
    #[builder(default = "None")]
    reference: Option<String>,
    /// Fully qualified url, e.g. https://example.com/ . Use this to override the callback url provided on the dashboard for this transaction
    #[builder(default = "None")]
    callback_url: Option<String>,
    /// If transaction is to create a subscription to a predefined plan, provide plan code here. This would invalidate the value provided in `amount`
    #[builder(default = "None")]
    plan: Option<String>,
    /// Number of times to charge customer during subscription to plan
    #[builder(default = "None")]
    invoice_limit: Option<u32>,
    /// Stringified JSON object of custom data. Kindly check the `Metadata` struct for more information.
    #[builder(default = "None")]
    metadata: Option<String>,
    /// An array of payment channels to control what channels you want to make available to the user to make a payment with.
    /// Available channels include: `["card", "bank", "ussd", "qr", "mobile_money", "bank_transfer", "eft"]`
    #[builder(default = "None")]
    channels: Option<Vec<Channel>>,
    /// The split code of the transaction split. e.g. `SPL_98WF13Eb3w`
    #[builder(default = "None")]
    split_code: Option<String>,
    /// The code for the subaccount that owns the payment. e.g. `ACCT_8f4s1eq7ml6rlzj`
    #[builder(default = "None")]
    subaccount: Option<String>,
    /// An amount used to override the split configuration for a single split payment.
    /// If set, the amount specified goes to the main account regardless of the split configuration.
    #[builder(default = "None")]
    transaction_charge: Option<u32>,
    /// Who bears Paystack charges? `account` or `subaccount` (defaults to account).
    #[builder(default = "None")]
    bearer: Option<String>
}

/// This struct is used to create a partial debit transaction body for creating a partial debit using the Paystack API.
/// This struct should be created using the `PartialDebitTransactionBodyBuilder`
/// The derive Builder allows for the automatic creation of the BuilderPattern
#[derive(Debug, Clone, Serialize, Default, Builder)]
pub struct PartialDebitTransactionBody {
    /// Authorization Code
    authorization_code: String,
    /// Specify the currency you want to debit. Allowed values are NGN or GHS.
    currency: Currency,
    /// Amount should be in the subunit of the supported currency
    amount: String,
    /// Customer's email address (attached to the authorization code)
    email: String,
    /// Unique transaction reference. Only `-`, `.`, `=` and alphanumeric characters allowed.
    #[builder(default = "None")]
    reference: Option<String>,
    /// Minimum amount to charge
    #[builder(default = "None")]
    at_least: Option<String>,
}
