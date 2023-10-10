//! Transaction Splits Models
//! ====================
//! This file contains the models for working with the transaction splits endpoint.
use crate::{BearerType, Currency, SplitType, SubaccountBody};
use derive_builder::Builder;
use serde::Serialize;

/// This struct is used to create a split payment on your integration.
/// The struct is constructed using the `CreateTransactionSplitBodyBuilder`
#[derive(Serialize, Debug, Default, Builder)]
pub struct CreateTransactionSplitBody<'a> {
    /// Name of the transaction split
    name: &'a str,
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
    bearer_subaccount: &'a str,
}

/// This struct is used to update a transaction split details on your integration.
/// The struct is constructed using the `UpdateTransactionSplitBodyBuilder`
#[derive(Serialize, Debug, Builder, Default)]
pub struct UpdateTransactionSplitBody<'a> {
    /// Name of the transaction split
    name: &'a str,
    /// True or False
    active: bool,
    /// Any of subaccount
    #[builder(default = "None")]
    bearer_type: Option<BearerType>,
    /// Subaccount code of a subaccount in the split group. This should be specified only if the `bearer_type is subaccount
    #[builder(default = "None")]
    bearer_subaccount: Option<SubaccountBody>,
}
