//! Subaccounts
//! ==============
//! This file contains the models for working with the subaccounts endpoint.

use crate::MetaData;
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// This struct is used to create the body for creating a subaccount on your integration.
#[derive(Serialize, Debug, Builder, Default)]
pub struct CreateSubaccountBody<'a> {
    /// Name of business for subaccount
    business_name: &'a str,
    /// Bank Code for the bank.
    /// You can get the list of Bank Codes by calling the List Banks endpoint.
    settlement_bank: &'a str,
    /// Bank Account Number
    account_number: &'a str,
    /// The default percentage charged when receiving on behalf of this subaccount
    percentage_charge: f32,
    /// A description for this subaccount
    description: &'a str,
    /// A contact email for the subaccount
    #[builder(default = "None")]
    primary_contact_email: Option<&'a str>,
    /// A name for the contact person for this subaccount
    #[builder(default = "None")]
    primary_contact_name: Option<&'a str>,
    /// A phone number to call for this subaccount
    #[builder(default = "None")]
    primary_contact_phone: Option<&'a str>,
    /// Stringified JSON object.
    /// Add a custom_fields attribute which has an array of objects if you would like the fields to be
    /// added to your transaction when displayed on the dashboard.
    /// Sample: {"custom_fields":[{"display_name":"Cart ID","variable_name": "cart_id","value": "8393"}]}
    #[builder(default = "None")]
    metadata: Option<&'a str>,
}

/// This struct represents the subaccount.
/// It can be used as the payload for the API end points that require a subaccount as a payload.
/// It is also possible to extract a single field from this struct to use as well.
/// The Struct is constructed using the `SubaccountBuilder`
#[derive(Serialize, Debug, Clone, Builder)]
pub struct SubaccountBody {
    /// This is the sub account code
    pub subaccount_code: String,
    /// This is the transaction share for the subaccount
    pub share: f32,
}

/// Represents the data of th Subaccounts
#[derive(Debug, Deserialize, Serialize)]
pub struct SubaccountData {
    /// Sub account data
    pub subaccount: SubaccountsResponseData,
    /// Share of split assigned to this sub
    pub share: u32,
}

/// Response from List Subaccount route
#[derive(Debug, Deserialize, Serialize)]
pub struct ListSubaccountsResponse {
    /// This lets you know if your request was successful or not.
    pub status: bool,
    /// This is a summary of the response and its status.
    pub message: String,
    /// This contain the results of your request.
    pub data: Vec<SubaccountsResponseData>,
    /// The meta key is used to provide context for the contents of the data key.
    pub meta: MetaData,
}

/// Data of the list Subaccount response
#[derive(Debug, Deserialize, Serialize)]
pub struct SubaccountsResponseData {
    /// Integration Id of subaccount.
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
    pub percentage_charge: f32,
    /// Verification status of subaccount.
    pub is_verified: Option<bool>,
    /// The name of the settlement bank for the subaccount.
    pub settlement_bank: String,
    /// The account number of the subaccount.
    pub account_number: String,
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
}

/// Represents the JSON response for subaccount creation.
#[derive(Debug, Deserialize, Serialize)]
pub struct CreateSubaccountResponse {
    /// The status of the JSON response.
    pub status: bool,
    /// The message associated with the JSON response
    pub message: String,
    /// Subaccount response data
    pub data: SubaccountsResponseData,
}

/// Represents the JSON response for fetch subaccount.
#[derive(Debug, Deserialize, Serialize)]
pub struct FetchSubaccountResponse {
    /// The status of the JSON response.
    pub status: bool,
    /// The message associated with the JSON response.
    pub message: String,
    /// Fetch Subaccount response data.
    pub data: SubaccountsResponseData,
}
