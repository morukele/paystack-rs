//! Transactions Models
//! ====================

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{Channel, Currency};

/// This struct is used to create a transaction body for creating a transaction using the Paystack API.
/// This struct is built using the `TransactionRequestBuilder` struct.
#[derive(Clone, Default, Debug, Serialize, Builder)]
pub struct TransactionRequest {
    /// Amount should be in the subunit of the supported currency
    pub amount: String,
    /// Customer's email address
    pub email: String,
    // optional parameters from here on
    /// The transaction currency. Defaults to your integration currency.
    #[builder(setter(into, strip_option), default)]
    pub currency: Option<Currency>,
    /// Unique transaction reference. Only `-`, `.`, `=` and alphanumeric characters allowed.
    #[builder(setter(into, strip_option), default)]
    pub reference: Option<String>,
    /// Fully qualified url, e.g. https://example.com/ . Use this to override the callback url provided on the dashboard for this transaction
    #[builder(setter(into, strip_option), default)]
    pub callback_url: Option<String>,
    /// If transaction is to create a subscription to a predefined plan, provide plan code here. This would invalidate the value provided in `amount`
    #[builder(setter(into, strip_option), default)]
    pub plan: Option<String>,
    /// Number of times to charge customer during subscription to plan
    #[builder(setter(into, strip_option), default)]
    pub invoice_limit: Option<u8>,
    /// Stringified JSON object of custom data. Kindly check the Metadata page for more information.
    #[builder(setter(into, strip_option), default)]
    pub metadata: Option<String>,
    /// An array of payment channels to control what channels you want to make available to the user to make a payment with.
    #[builder(setter(into, strip_option), default)]
    pub channel: Option<Vec<Channel>>,
    /// The split code of the transaction split. e.g. `SPL_98WF13Eb3w`
    #[builder(setter(into, strip_option), default)]
    pub split_code: Option<String>,
    /// The code for the subaccount that owns the payment. e.g. `ACCT_8f4s1eq7ml6rlzj`
    #[builder(setter(into, strip_option), default)]
    pub subaccount: Option<String>,
    /// An amount used to override the split configuration for a single split payment.
    /// If set, the amount specified goes to the main account regardless of the split configuration.
    #[builder(setter(into, strip_option), default)]
    pub transaction_charge: Option<String>,
    /// Use this param to indicate who bears the transaction charges. Allowed values are: `account` or `subaccount` (defaults to `account`).
    #[builder(setter(into, strip_option), default)]
    pub bearer: Option<String>,
}

/// This struct represents the data of the transaction response.
#[derive(Deserialize, Debug, Clone)]
pub struct TransactionResponseData {
    /// Generated URL to authorize the transaction.
    pub authorization_url: String,
    /// Access code of the transaction.
    pub access_code: String,
    /// Reference of the transaction.
    pub reference: String,
}

#[cfg(test)]
mod test {
    use super::*;
    use std::error::Error;

    #[test]
    fn can_create_transaction_body_with_builder() -> Result<(), Box<dyn Error>> {
        let transaction = TransactionRequestBuilder::default()
            .amount(String::from("10000"))
            .email(String::from("email@example.com"))
            .currency(Currency::NGN)
            .build()?;

        assert_eq!(transaction.email, "email@example.com");
        assert_eq!(transaction.amount, "10000");
        assert_eq!(transaction.currency, Some(Currency::NGN));
        assert_eq!(transaction.bearer, None);

        Ok(())
    }

    #[test]
    fn cannot_create_transaction_body_without_compulsory_field() -> Result<(), Box<dyn Error>> {
        let transaction = TransactionRequestBuilder::default()
            .currency(Currency::GHS)
            .build();

        assert!(transaction.is_err());

        Ok(())
    }
}
