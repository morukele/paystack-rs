//! Plans Models
//! =============
//! This file contains the models and options for the Plans endpoint of the Paystack API

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

/// Request body to create a plan on your integration.
/// Should be created via `PlanRequestBuilder`
#[derive(Clone, Default, Debug, Serialize, Deserialize)]
pub struct PlanRequest {
    /// Name of plan
    pub name: String,
    pub amount: u64,
}
