use std::sync::Arc;

use super::PAYSTACK_BASE_URL;
use crate::{
    HttpClient, PaystackAPIError, PaystackResult, PlanRequest, PlanResponseData, Response,
};

pub struct PlansEndpoints<T: HttpClient + Default> {
    /// Paystack API Key
    key: String,
    /// Base URL for the plans route
    base_url: String,
    /// Http client for the route
    http: Arc<T>,
}

/// Create a new `PlansEndpoints<T>` instance
///
/// # Arguments
/// - `key` - The Paystack API key
/// - `http`: The HTTP client implementation to use for the API requests
///
/// # Returns
/// A new PlansEndpoints instance
impl<T: HttpClient + Default> PlansEndpoints<T> {
    pub fn new(key: Arc<String>, http: Arc<T>) -> PlansEndpoints<T> {
        let base_url = format!("{PAYSTACK_BASE_URL}/plan");
        PlansEndpoints {
            key: key.to_string(),
            base_url,
            http,
        }
    }

    /// Create a plan on your integration
    ///
    /// # Arguments
    /// * `plan_request` - The request data to create the plan.
    ///   Should be created with a `PlanRequestBuilder` struct.
    ///
    /// # Returns
    /// A result containing the plan response data or an error  
    pub async fn create_plan(&self, plan_request: PlanRequest) -> PaystackResult<PlanResponseData> {
        let url = &self.base_url;
        let body = serde_json::to_value(plan_request)
            .map_err(|e| PaystackAPIError::Plan(e.to_string()))?;

        let response = self
            .http
            .post(url, &self.key, &body)
            .await
            .map_err(|e| PaystackAPIError::Plan(e.to_string()))?;

        let parsed_response: Response<PlanResponseData> =
            serde_json::from_str(&response).map_err(|e| PaystackAPIError::Plan(e.to_string()))?;

        Ok(parsed_response)
    }
}
