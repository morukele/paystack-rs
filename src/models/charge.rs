//! Charge
//! ===========
//! This file contains all the structs and definitions needed to
//! create charges using the Paystack API.

use crate::{Channel, Currency};
use derive_builder::Builder;
use serde::Serialize;

/// This struct is used to create a charge body for creating a Charge Authorization using the Paystack API.
/// The struct is constructed using the `ChargeBodyBuilder`
#[derive(Serialize, Debug, Builder)]
pub struct ChargeRequest {
    /// Customer's email address
    email: String,
    /// Amount should be in the smallest unit of the currency e.g. kobo if in NGN and cents if in USD
    amount: String,
    /// Valid authorization code to charge
    authorization_code: String,
    /// Unique transaction reference. Only `-`, `.`, `=` and alphanumeric characters allowed.
    #[builder(default = "None")]
    reference: Option<String>,
    /// Currency in which amount should be charged.
    #[builder(default = "None")]
    currency: Option<Currency>,
    /// Stringified JSON object.
    /// Add a custom_fields attribute which has an array of objects if you would like the fields to be added to your transaction
    /// when displayed on the dashboard.
    /// Sample: {"custom_fields":[{"display_name":"Cart ID","variable_name": "cart_id","value": "8393"}]}
    #[builder(default = "None")]
    metadata: Option<String>,
    /// Send us 'card' or 'bank' or 'card','bank' as an array to specify what options to show the user paying
    #[builder(default = "None")]
    channel: Option<Vec<Channel>>,
    /// The code for the subaccount that owns the payment. e.g. `ACCT_8f4s1eq7ml6rlzj`
    #[builder(default = "None")]
    subaccount: Option<String>,
    /// A flat fee to charge the subaccount for this transaction in the subunit of the supported currency.
    /// This overrides the split percentage set when the subaccount was created.
    /// Ideally, you will need to use this if you are splitting in flat rates (since subaccount creation only allows for percentage split).
    #[builder(default = "None")]
    transaction_charge: Option<u32>,
    /// Who bears Paystack charges? account or subaccount (defaults to account).
    #[builder(default = "None")]
    bearer: Option<String>,
    /// If you are making a scheduled charge call, it is a good idea to queue them so the processing system does not
    /// get overloaded causing transaction processing errors.
    /// Send queue:true to take advantage of our queued charging.
    #[builder(default = "None")]
    queue: Option<bool>,
}
