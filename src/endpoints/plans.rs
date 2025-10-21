use std::sync::Arc;

use super::PAYSTACK_BASE_URL;
use crate::{
    HttpClient, Interval, PaystackAPIError, PaystackResult, PlanRequest, PlanResponseData,
    PlanStatus, Response,
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

    /// Lists plans available in your integration
    ///
    /// # Arguments
    /// * `per_page` - specify how many records you want to retrieve per page. Defaults to 50 if None
    /// * `page` - specify exactly what page you want to retrieve. Defaults to 1 if None
    /// * `status` - Optional parameter to filter list by plans with specified status
    /// * `interval` - Optional parameter to filter list by plans with specified interval
    /// * `amount`- Optional parameter to filter list by plans with specified amount using the supported currency
    pub async fn list_plans(
        &self,
        per_page: Option<u8>,
        page: Option<u8>,
        status: Option<PlanStatus>,
        interval: Option<Interval>,
        amount: Option<u32>,
    ) -> PaystackResult<Vec<PlanResponseData>> {
        let url = &self.base_url;

        let per_page = per_page.unwrap_or(50).to_string();
        let page = page.unwrap_or(1).to_string();

        let mut query = vec![("perPage", per_page), ("page", page)];

        // Process optional parameters
        if let Some(s) = status {
            query.push(("status", s.to_string()));
        }

        if let Some(i) = interval {
            query.push(("interval", i.to_string()));
        }

        if let Some(a) = amount {
            query.push(("amount", a.to_string()));
        }

        // convert all string to &str
        // TODO: there has to be a cleaner way of doing this
        let query: Vec<(&str, &str)> = query.iter().map(|(k, v)| (*k, v.as_str())).collect();

        dbg!("{:?}", &query);

        let response = self
            .http
            .get(url, &self.key, Some(&query))
            .await
            .map_err(|e| PaystackAPIError::Plan(e.to_string()))?;

        let parsed_response: Response<Vec<PlanResponseData>> =
            serde_json::from_str(&response).map_err(|e| PaystackAPIError::Plan(e.to_string()))?;

        Ok(parsed_response)
    }

    pub async fn fetch_plan() {
        todo!()
    }

    pub async fn update_plan() {
        todo!()
    }
}
