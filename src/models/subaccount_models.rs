//! Subaccounts
//! ==============
//! This file contains the models for working with the subaccounts endpoint.

use super::Currency;
use crate::utils::bool_from_int_or_bool;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// This struct is used to create the body for creating a subaccount on your integration.
/// Use the `SubaccountRequestBuilder` to create this object.
#[derive(Serialize, Debug, Builder, Default)]
pub struct CreateSubaccountRequest {
    /// Name of business for subaccount
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    business_name: Option<String>,
    /// Bank Code for the bank.
    /// You can get the list of Bank Codes by calling the List Banks endpoint.
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    settlement_bank: Option<String>,
    /// Bank Account Number
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    account_number: Option<String>,
    /// The default percentage charged when receiving on behalf of this subaccount
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    percentage_charge: Option<f32>,
    /// A description for this subaccount
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    /// A contact email for the subaccount
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_contact_email: Option<String>,
    /// A name for the contact person for this subaccount
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_contact_name: Option<String>,
    /// A phone number to call for this subaccount
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    primary_contact_phone: Option<String>,
    /// Stringified JSON object.
    /// Add a custom_fields attribute which has an array of objects if you would like the fields to be
    /// added to your transaction when displayed on the dashboard.
    /// Sample: {"custom_fields":[{"display_name":"Cart ID","variable_name": "cart_id","value": "8393"}]}
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    metadata: Option<String>,
}

/// This struct represents the subaccount.
/// It can be used as the payload for the API end points that require a subaccount as a payload.
/// It is also possible to extract a single field from this struct to use as well.
/// The Struct is constructed using the `SubaccountBodyBuilder`
#[derive(Serialize, Debug, Clone, Builder, Default)]
pub struct SubaccountBody {
    /// This is the subaccount code
    pub subaccount: String,
    /// This is the transaction share for the subaccount
    pub share: f32,
}

/// Represents the data of th Subaccounts
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct SubaccountData {
    /// Sub account data
    pub subaccount: SubaccountsResponseData,
    /// Share of split assigned to this sub
    pub share: u32,
}

/// Data of the list Subaccount response
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct SubaccountsResponseData {
    /// Integration ID of subaccount.
    pub integration: Option<u32>,
    /// Subaccount domain.
    pub domain: Option<String>,
    /// The code of the subaccount.
    pub subaccount_code: String,
    /// The name of the business associated with the subaccount.
    pub business_name: String,
    /// The description of the business associated with the subaccount.
    pub description: Option<String>,
    /// The name of the primary contact for the business, if available.
    pub primary_contact_name: Option<String>,
    /// The email of the primary contact for the business, if available.
    pub primary_contact_email: Option<String>,
    /// The phone number of the primary contact for the business, if available.
    pub primary_contact_phone: Option<String>,
    /// Additional metadata associated with the subaccount, if available.
    pub metadata: Option<String>,
    /// The percentage charge for transactions associated with the subaccount.
    pub percentage_charge: Option<f32>,
    /// Verification status of subaccount.
    pub is_verified: Option<bool>,
    /// The name of the settlement bank for the subaccount.
    pub settlement_bank: String,
    /// The id of the settlement bank for the subaccount.
    pub bank_id: Option<u32>,
    /// The account number of the subaccount.
    pub account_number: String,
    /// Currency of the subaccount
    pub currency: Option<Currency>,
    /// If the account is active or not, should be 1 for active and 0 for inactive
    #[serde(default, deserialize_with = "bool_from_int_or_bool")]
    pub active: Option<bool>,
    /// Settlement schedule of subaccount.
    pub settlement_schedule: Option<String>,
    /// The ID of the subaccount.
    pub id: u32,
    /// Creation time of subaccount.
    #[serde(rename = "createdAt")]
    pub created_at: Option<String>,
    /// Last update time of subaccount.
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<String>,
    pub product: Option<String>,
    pub managed_by_integration: Option<u32>,
}

/// This struct is used to create the body for deleting a subaccount on your integration.
#[derive(Debug, Deserialize, Serialize, Builder, Default)]
pub struct DeleteSubAccountBody {
    /// This is the subaccount code
    pub subaccount: String,
}
