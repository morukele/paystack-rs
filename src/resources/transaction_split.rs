//! Transaction Split
//! =================
//! The Transaction Splits API enables merchants split the settlement for a transaction
//! across their payout account, and one or more subaccounts.

use crate::{BearerType, Currency, SplitType};
use derive_builder::Builder;
use serde::Serialize;

/// This struct is used to create a split payment on your integration.
/// The struct is constructed using the `CreateTransactionSplitBodyBuilder`
#[derive(Serialize, Debug, Default, Builder)]
pub struct CreateTransactionSplitBody {
    /// Name of the transaction split
    name: String,
    /// The type of transaction split you want to create
    #[serde(rename = "type")]
    split_type: SplitType,
    /// Any of the supported currency
    currency: Currency,
    /// A list of object containing subaccount code and number of shares: `[{subaccount: ‘ACT_xxxxxxxxxx’, share: xxx},{...}]`
    subaccounts: Vec<Subaccount>,
    /// Any of subaccount
    bearer_type: BearerType,
    /// Subaccount code
    bearer_subaccount: String,
}

/// This struct represents the subaccount.
/// It can be used as the payload for the API end points that require a subaccount as a payload.
/// It is also possible to extract a single field from this struct to use as well.
/// The Struct is constructed using the `SubaccountBuilder`
#[derive(Serialize, Debug, Clone, Builder)]
pub struct Subaccount {
    /// This is the sub account code
    pub subaccount: String,
    /// This is the transaction share for the subaccount
    pub share: u32,
}

/// This struct is used to update a transaction split details on your integration.
/// The struct is constructed using the `UpdateTransactionSplitBodyBuilder`
#[derive(Serialize, Debug, Builder)]
pub struct UpdateTransactionSplitBody {
    /// Name of the transaction split
    pub name: String,
    /// True or False
    pub active: bool,
    /// Any of subaccount
    #[builder(default = "None")]
    pub bearer_type: Option<BearerType>,
    /// Subaccount code of a subaccount in the split group. This should be specified only if the `bearer_type is subaccount
    #[builder(default = "None")]
    pub bearer_subaccount: Option<Subaccount>,
}
