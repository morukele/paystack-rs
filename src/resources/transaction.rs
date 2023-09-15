//! Transactions
//! =============
//! This file contains all the structs and definitions needed to
//! create a transaction using the paystack API.

use crate::{Channel, Currency};
use serde::Serialize;

/// This struct is used to create a transaction body for creating a transaction using the Paystack API.
#[derive(Serialize, Debug, Default)]
pub struct InitializeTransactionBody {
    /// Amount should be in the smallest unit of the currency e.g. kobo if in NGN and cents if in USD
    pub amount: String,
    /// Customer's email address
    pub email: String,
    /// Currency in which amount should be charged (NGN, GHS, ZAR or USD). Defaults to your integration currency.
    pub currency: Option<Currency>,
    /// Unique transaction reference. Only -, ., = and alphanumeric characters allowed.
    pub reference: Option<String>,
    /// Fully qualified url, e.g. https://example.com/ . Use this to override the callback url provided on the dashboard for this transaction
    pub callback_url: Option<String>,
    /// If transaction is to create a subscription to a predefined plan, provide plan code here. This would invalidate the value provided in `amount`
    pub plan: Option<String>,
    /// Number of times to charge customer during subscription to plan
    pub invoice_limit: Option<u32>,
    /// Stringified JSON object of custom data. Kindly check the `Metadata` struct for more information.
    pub metadata: Option<String>,
    /// An array of payment channels to control what channels you want to make available to the user to make a payment with.
    /// Available channels include: `["card", "bank", "ussd", "qr", "mobile_money", "bank_transfer", "eft"]`
    pub channels: Option<Vec<Channel>>,
    /// The split code of the transaction split. e.g. `SPL_98WF13Eb3w`
    pub split_code: Option<String>,
    /// The code for the subaccount that owns the payment. e.g. `ACCT_8f4s1eq7ml6rlzj`
    pub subaccount: Option<String>,
    /// An amount used to override the split configuration for a single split payment.
    /// If set, the amount specified goes to the main account regardless of the split configuration.
    pub transaction_charge: Option<u32>,
    /// Who bears Paystack charges? `account` or `subaccount` (defaults to account).
    pub bearer: Option<String>
}

/// This struct is used to create a partial debit transaction body for creating a partial debit using the Paystack API.
#[derive(Debug, Clone, Serialize, Default)]
pub struct PartialDebitTransactionBody {
    /// Authorization Code
    pub authorization_code: String,
    /// Specify the currency you want to debit. Allowed values are NGN or GHS.
    pub currency: Currency,
    /// Amount should be in the subunit of the supported currency
    pub amount: String,
    /// Customer's email address (attached to the authorization code)
    pub email: String,
    /// Unique transaction reference. Only `-`, `.`, `=` and alphanumeric characters allowed.
    pub reference: Option<String>,
    /// Minimum amount to charge
    pub at_least: Option<String>,
}
