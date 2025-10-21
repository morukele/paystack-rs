//! Transaction Split Models
//! ========================
//! This file contains the models for working with the transaction splits endpoint.

use crate::{BearerType, Currency, Domain, SplitType, SubaccountBody, SubaccountData};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// This struct is used to create a split payment on your integration.
/// The struct is constructed using the `TransactionSplitRequestBuilder`
#[derive(Serialize, Debug, Default, Builder)]
pub struct TransactionSplitRequest {
    /// Name of the transaction split
    name: String,
    /// The type of transaction split you want to create
    #[serde(rename = "type")]
    split_type: SplitType,
    /// Any of the supported currency
    currency: Currency,
    /// A list of object containing subaccount code and number of shares: `[{subaccount: ‘ACT_xxxxxxxxxx’, share: xxx},{...}]`
    subaccounts: Vec<SubaccountBody>,
    /// Any of subaccount
    bearer_type: BearerType,
    /// Subaccount code
    bearer_subaccount: String,
}

/// Represents the percentage split data received in the JSON response.
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct TransactionSplitResponseData {
    /// The ID of the percentage split.
    pub id: u32,
    /// The name of the percentage split.
    pub name: String,
    /// The type of the percentage split.
    #[serde(rename = "type")]
    pub split_type: String,
    /// The currency used for the percentage split.
    pub currency: String,
    /// The integration associated with the percentage split.
    pub integration: u32,
    /// The domain associated with the percentage split.
    pub domain: Domain,
    /// The split code of the percentage split.
    pub split_code: String,
    /// Indicates whether the percentage split is active or not.
    #[serde(default)]
    pub active: Option<bool>,
    /// The bearer type of the percentage split.
    pub bearer_type: String,
    /// The subaccount ID of the bearer associated with the percentage split.
    pub bearer_subaccount: u32,
    /// The creation timestamp of the percentage split.
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    /// The last update timestamp of the percentage split.
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
    pub is_dynamic: Option<bool>,
    /// The list of subaccounts involved in the percentage split.
    pub subaccounts: Vec<SubaccountData>,
    /// The total count of subaccounts in the percentage split.
    pub total_subaccounts: u32,
}

/// This struct is used to update a transaction split details on your integration.
/// The struct is constructed using the `UpdateTransactionSplitRequestBuilder`
#[derive(Serialize, Debug, Builder, Default)]
pub struct UpdateTransactionSplitRequest {
    /// Name of the transaction split
    name: String,
    /// True or False
    active: bool,
    /// Any of subaccount
    #[builder(setter(strip_option), default)]
    bearer_type: Option<BearerType>,
    /// Subaccount code of a subaccount in the split group. This should be specified only if the `bearer_type is subaccount
    #[builder(setter(strip_option), default)]
    bearer_subaccount: Option<SubaccountBody>,
}
