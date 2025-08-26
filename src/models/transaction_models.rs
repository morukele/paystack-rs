//! Transactions Models
//! ====================

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::{Authorization, Channel, Currency, CustomerResponseData};

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
    #[builder(setter(strip_option), default)]
    pub currency: Option<Currency>,
    /// Unique transaction reference. Only `-`, `.`, `=` and alphanumeric characters allowed.
    #[builder(setter(strip_option), default)]
    pub reference: Option<String>,
    /// Fully qualified url, e.g. https://example.com/ . Use this to override the callback url provided on the dashboard for this transaction
    #[builder(setter(strip_option), default)]
    pub callback_url: Option<String>,
    /// If transaction is to create a subscription to a predefined plan, provide plan code here. This would invalidate the value provided in `amount`
    #[builder(setter(strip_option), default)]
    pub plan: Option<String>,
    /// Number of times to charge customer during subscription to plan
    #[builder(setter(strip_option), default)]
    pub invoice_limit: Option<u8>,
    /// Stringified JSON object of custom data. Kindly check the Metadata page for more information.
    #[builder(setter(strip_option), default)]
    pub metadata: Option<String>,
    /// An array of payment channels to control what channels you want to make available to the user to make a payment with.
    #[builder(setter(strip_option), default)]
    pub channel: Option<Vec<Channel>>,
    /// The split code of the transaction split. e.g. `SPL_98WF13Eb3w`
    #[builder(setter(strip_option), default)]
    pub split_code: Option<String>,
    /// The code for the subaccount that owns the payment. e.g. `ACCT_8f4s1eq7ml6rlzj`
    #[builder(setter(strip_option), default)]
    pub subaccount: Option<String>,
    /// An amount used to override the split configuration for a single split payment.
    /// If set, the amount specified goes to the main account regardless of the split configuration.
    #[builder(setter(strip_option), default)]
    pub transaction_charge: Option<String>,
    /// Use this param to indicate who bears the transaction charges. Allowed values are: `account` or `subaccount` (defaults to `account`).
    #[builder(setter(strip_option), default)]
    pub bearer: Option<String>,
}

/// This struct is used to create a partial debit transaction body for creating a partial debit using the Paystack API.
/// This struct should be created using the `PartialDebitTransactionRequestBuilder`
/// The derive Builder allows for the automatic creation of the BuilderPattern
#[derive(Debug, Clone, Serialize, Default, Builder)]
pub struct PartialDebitTransactionRequest {
    /// Authorization Code
    authorization_code: String,
    /// Specify the currency you want to debit. Allowed values are NGN or GHS.
    currency: Currency,
    /// Amount should be in the subunit of the supported currency
    amount: String,
    /// Customer's email address (attached to the authorization code)
    email: String,
    /// Unique transaction reference. Only `-`, `.`, `=` and alphanumeric characters allowed.
    #[builder(default)]
    reference: Option<String>,
    /// Minimum amount to charge
    #[builder(default)]
    at_least: Option<String>,
}

/// This struct represents the data of the transaction response.
#[derive(Deserialize, Debug, Clone, Default)]
pub struct TransactionResponseData {
    /// Generated URL to authorize the transaction.
    pub authorization_url: String,
    /// Access code of the transaction.
    pub access_code: String,
    /// Reference of the transaction.
    pub reference: String,
}

/// This struct represents the data of the transaction status response.
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct TransactionStatusData {
    /// Id of the Transaction
    pub id: u64,
    /// Status of the Transaction. It can be `success`, `abandoned` or `failed`
    pub status: String,
    /// Reference of the Transaction
    pub reference: String,
    /// Amount of the transaction in the lowest denomination of the currency e.g. Kobo for NGN and cent for USD.
    pub amount: u32,
    /// Message from the transaction.
    pub message: Option<String>,
    /// Response from the payment gateway.
    pub gateway_response: String,
    /// Time the Transaction was completed.
    pub paid_at: Option<String>,
    /// Time the Transaction was created.
    pub created_at: String,
    /// Transaction channel. It can be `card` or `bank`.
    pub channel: String,
    /// Currency code of the Transaction e.g. `NGN for Nigerian Naira` and `USD for US Dollar`.
    pub currency: String,
    /// IP address of the computers the Transaction has passed through.
    pub ip_address: Option<String>,
    /// Meta data associated with the Transaction.
    pub metadata: Option<String>,
    /// Transaction fees to override the default fees specified in the integration.
    pub fees: Option<i32>,
    /// Transaction customer data.
    pub customer: CustomerResponseData,
    /// Transaction authorization data.
    pub authorization: Authorization,
}

/// This struct represents the transaction timeline data.
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct TransactionTimelineData {
    /// Time spent in carrying out the transaction in ms.
    pub time_spent: Option<u32>,
    /// Number of attempts for the transaction.
    pub attempts: Option<u32>,
    /// Authentication use for the transaction.
    pub authentication: Option<String>,
    /// Number of errors for the transaction.
    pub errors: Option<u32>,
    /// Success status of the transaction.
    pub success: Option<bool>,
    /// If transaction was carried out with mobile.
    pub mobile: Option<bool>,
    /// Transaction inputs i.e. messages associated with the transaction.
    pub input: Option<String>,
    /// Transaction channel.
    pub channel: Option<String>,
    /// Transaction history.
    pub history: Option<Vec<TransactionHistoryResponse>>,
}

/// This struct represents the transaction history data
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct TransactionHistoryResponse {
    /// Transaction action.
    #[serde(rename = "type")]
    pub action_type: String,
    /// Description of the action.
    pub message: String,
    /// Time action was taken in ms.
    pub time: u32,
}

/// Transaction total data.
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct TransactionTotalData {
    /// Total number of transactions in the integration.
    pub total_transactions: Option<u32>,
    /// Total of unique number of customers in the integration.
    pub unique_customers: Option<u32>,
    /// Total volume of transaction in the integration.
    pub total_volume: Option<u32>,
    /// Total volume of transaction broken down by currency.
    pub total_volume_by_currency: Option<Vec<VolumeByCurrency>>,
    /// Total volume of pending transfers.
    pub pending_transfers: Option<u32>,
    /// Total volume of pending transfer broken down by currency.
    pub pending_transfers_by_currency: Option<Vec<VolumeByCurrency>>,
}

/// Transaction volume by currency.
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct VolumeByCurrency {
    /// Currency code.
    pub currency: String,
    /// Amount in the lowest denomination of the currency.
    pub amount: u32,
}

/// Export transaction response data.
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ExportTransactionData {
    /// Path to download the exported transaction file.
    pub path: String,
}

/// Transaction identifier.
///
/// It can either be a transaction reference or a transaction ID
pub enum TransactionIdentifier {
    Id(u64),
    Reference(String),
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
