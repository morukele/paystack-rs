use std::fmt;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{Authorization, Domain};

/// This struct is used to create a subscription body for creating a subscription using the Paystack API.
/// This struct is built using the `SubscriptionRequestBuilder` struct.
#[derive(Clone, Default, Debug, Serialize, Builder)]
pub struct SubscriptionRequest {
    /// Customer's email address or customer code
    pub customer: String,
    /// Plan code
    pub plan: String,
    /// If customer has multiple authorizations, you can set the desired authorization you wish to use for this subscription here. If this is not supplied, the customer's most recent authorization would be used
    pub authorization: Option<String>,
    /// Set the date for the first debit. (ISO 8601 format) e.g. `2017-05-16T00:30:13+01:00`
    pub start_data: Option<OffsetDateTime>,
}

#[derive(Debug, Clone, Deserialize, Serialize, Default)]
pub struct SubscriptionResponseData {
    pub customer: u32,
    pub plan: u32,
    pub integration: u32,
    pub domain: Domain,
    pub start: u32,
    pub status: SubscriptionStatus,
    pub quantity: u32,
    pub amount: u32,
    pub subscription_code: String,
    pub email_token: String,
    pub authorization: Authorization,
    pub easy_cron_id: Option<String>,
    pub cron_expression: String,
    pub next_payment_date: String,
    pub open_invoice: Option<String>,
    pub id: u32,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum SubscriptionStatus {
    #[default]
    Complete,
    Active,
}

impl fmt::Display for SubscriptionStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = match self {
            SubscriptionStatus::Complete => "complete",
            SubscriptionStatus::Active => "active",
        };
        write!(f, "{status}")
    }
}
