//! Plans Models
//! =============
//! This file contains the models and options for the Plans endpoint of the Paystack API

use std::fmt;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::utils::string_or_number_to_u32;
use crate::{Currency, Domain, Subscription};

/// Request body to create a plan on your integration.
/// Should be created via `PlanRequestBuilder`
#[derive(Clone, Default, Debug, Serialize, Deserialize, Builder)]
pub struct PlanRequest {
    /// Name of plan
    pub name: String,
    /// Amount for the plan. Should be in the subunit of the supported currency
    // NB: accepting string here to be consistent with the api.
    pub amount: String,
    /// Interval in words, Use the `Interval` Enum for valid options.
    pub interval: Interval,
    /// A description of this plan
    #[builder(setter(strip_option))]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Set to false if you don't want invoices to be sent to your customers
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_invoice: Option<bool>,
    /// Set to false if you don't want text messages to be sent to your customers
    // NB: docs says string, but should be bool.
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_sms: Option<bool>,
    /// Currency in which the amount is set.
    /// Defaults to the Default Currency of the integration
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Currency>,
    /// Number of invoices to raise during subscription to this plan.
    /// Can be overridden by specifying an `invoice_limit` while subscribing.
    #[builder(setter(strip_option), default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_limit: Option<u8>,
}

/// Options for the different payment intervals for plans supported by the paystack API.
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Interval {
    Daily,
    Weekly,
    #[default]
    Monthly,
    Quarterly,
    /// Every 6 months
    Biannually,
    Annually,
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let interval = match self {
            Interval::Daily => "daily",
            Interval::Weekly => "weekly",
            Interval::Monthly => "monthly",
            Interval::Quarterly => "quarterly",
            Interval::Biannually => "biannually",
            Interval::Annually => "annually",
        };
        write!(f, "{interval}")
    }
}

// TODO: figure out the the other plan status
#[derive(Debug, Serialize, Deserialize, Clone, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum PlanStatus {
    #[default]
    Active,
    Archived,
    Deleted,
}

impl fmt::Display for PlanStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let plan_status = match self {
            PlanStatus::Active => "Active",
            PlanStatus::Archived => "Archived",
            PlanStatus::Deleted => "Deleted",
        };
        write!(f, "{plan_status}")
    }
}

/// This struct represents the data of the create plan response.
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct PlanResponseData {
    pub subscriptions: Option<Vec<Subscription>>,
    pub name: String,
    #[serde(deserialize_with = "string_or_number_to_u32")]
    pub amount: u32,
    pub interval: Interval,
    pub integration: u32,
    pub domain: Domain,
    pub plan_code: String,
    pub description: Option<String>,
    pub send_invoices: Option<bool>,
    pub send_sms: bool,
    pub hosted_page: bool,
    pub hosted_page_url: Option<String>,
    pub hosted_page_summary: Option<String>,
    pub currency: Currency,
    pub id: u32,
    #[serde(rename = "createdAt")]
    pub created_at: String,
    #[serde(rename = "updatedAt")]
    pub updated_at: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;

    #[test]
    fn can_create_plan_request_with_builder() -> Result<(), Box<dyn Error>> {
        let plan = PlanRequestBuilder::default()
            .name("test plan".to_string())
            .amount("100000".to_string())
            .interval(Interval::Monthly)
            .description("some description".to_string())
            .build()?;

        assert_eq!(plan.name, "test plan");
        assert_eq!(plan.amount, "100000");
        assert_eq!(plan.interval, Interval::Monthly);
        assert_eq!(plan.description, Some("some description".to_string()));

        Ok(())
    }

    #[test]
    fn cannot_create_plan_request_without_compulsory_field() -> Result<(), Box<dyn Error>> {
        let plan = PlanRequestBuilder::default()
            .currency(Currency::XOF)
            .build();

        assert!(plan.is_err());

        Ok(())
    }
}
