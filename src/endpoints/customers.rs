//! Customers
//! =========
//! Thse Customers API allows you to create and maange customers on your integration

use super::BASE_URL;
use crate::{
    CreateCustomerRequest, CustomerResponseData, HttpClient, PaystackAPIError, PaystackResult,
    Response, RiskAction, UpdateCustomerRequest, ValidateCustomerRequest,
};
use serde_json::json;
use std::{marker::PhantomData, sync::Arc};

/// A struct to hold all the functions of the customers API endpoint
#[derive(Debug, Clone)]
pub struct CustomersEndpoints<T: HttpClient + Default> {
    /// Paystack API key
    key: String,
    /// Base URL for the customer route
    base_url: String,
    /// Http client for the route
    http: Arc<T>,
}

impl<T: HttpClient + Default> CustomersEndpoints<T> {
    /// Creates a new CustomersEndpoints instance
    ///
    /// # Arguments
    /// * `key` - The Paystack API key
    /// * `http` - The HTTP client implementation to use for API requests
    ///
    /// # Returns
    /// A new CustomersEndpoints instance
    pub fn new(key: Arc<String>, http: Arc<T>) -> CustomersEndpoints<T> {
        let base_url = format!("{}/customer", BASE_URL);
        CustomersEndpoints {
            key: key.to_string(),
            base_url,
            http,
        }
    }

    /// Create customer on your integration
    ///
    /// # Arguments
    /// * `create_customer_request` - Contains the information about the customer to be created.
    ///   It should be built with `CreateCustomerRequestBuilder`.
    ///
    /// # Returns
    /// A Result containing the customer response data or an error
    pub async fn create_customer(
        &self,
        create_customer_request: CreateCustomerRequest,
    ) -> PaystackResult<CustomerResponseData> {
        let url = &self.base_url;
        let body = serde_json::to_value(create_customer_request)
            .map_err(|e| PaystackAPIError::Customer(e.to_string()))?;

        let response = self
            .http
            .post(&url, &self.key, &body)
            .await
            .map_err(|e| PaystackAPIError::Customer(e.to_string()))?;

        let parsed_response: Response<CustomerResponseData> = serde_json::from_str(&response)
            .map_err(|e| PaystackAPIError::Customer(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Lists customers available on your integration
    ///
    /// # Arguments
    /// * `per_page` - Optional number of records to retrieve per page. Default is 50
    /// * `page` - Optional page number to retrieve. Default is 1
    ///
    /// # Returns
    /// A Result containing a vector of customer response data or an error
    pub async fn list_customers(
        &self,
        per_page: Option<u8>,
        page: Option<u8>,
    ) -> PaystackResult<Vec<CustomerResponseData>> {
        let url = &self.base_url;

        let per_page = per_page.unwrap_or(50).to_string();
        let page = page.unwrap_or(1).to_string();
        let query = vec![("perPage", per_page.as_str()), ("page", page.as_str())];

        let response = self
            .http
            .get(&url, &self.key, Some(&query))
            .await
            .map_err(|e| PaystackAPIError::Customer(e.to_string()))?;

        let parsed_response: Response<Vec<CustomerResponseData>> = serde_json::from_str(&response)
            .map_err(|e| PaystackAPIError::Customer(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Gets details of a customer on your integration
    ///
    /// # Arguments
    /// * `email_or_code` - Email or customer code for the customer to fetch
    ///
    /// # Returns
    /// A Result containing the customer response data or an error
    pub async fn fetch_customer(
        &self,
        email_or_code: String,
    ) -> PaystackResult<CustomerResponseData> {
        let url = format!("{}/{}", self.base_url, email_or_code);

        let response = self
            .http
            .get(&url, &self.key, None)
            .await
            .map_err(|e| PaystackAPIError::Customer(e.to_string()))?;

        let parsed_response: Response<CustomerResponseData> = serde_json::from_str(&response)
            .map_err(|e| PaystackAPIError::Customer(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Updates a customer's details on your integration
    ///
    /// # Arguments
    /// * `customer_code` - The customer's code
    /// * `update_customer_request` - The data to update the customer with.
    ///   Should be created with the UpdateCustomerRequestBuilder struct
    ///
    /// # Returns
    /// A Result containing the updated customer response data or an error
    pub async fn update_customer(
        &self,
        customer_code: String,
        update_customer_request: UpdateCustomerRequest,
    ) -> PaystackResult<CustomerResponseData> {
        let url = format!("{}/{}", self.base_url, customer_code);
        let body = serde_json::to_value(update_customer_request)
            .map_err(|e| PaystackAPIError::Customer(e.to_string()))?;

        let response = self
            .http
            .put(&url, &self.key, &body)
            .await
            .map_err(|e| PaystackAPIError::Customer(e.to_string()))?;

        let parsed_response: Response<CustomerResponseData> = serde_json::from_str(&response)
            .map_err(|e| PaystackAPIError::Customer(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Validates a customer's identity
    ///
    /// # Arguments
    /// * `customer_code` - Email or customer code of customer to be identified
    /// * `customer_validation_request` - The data to validate the customer with.
    ///   Should be created with the ValidateCustomerRequestBuilder struct
    ///
    /// # Returns
    /// A Result containing the validation response or an error
    pub async fn validate_customer(
        &self,
        customer_code: String,
        customer_validation_request: ValidateCustomerRequest,
    ) -> PaystackResult<PhantomData<String>> {
        let url = format!("{}/{}/identification", self.base_url, customer_code);
        let body = serde_json::to_value(customer_validation_request)
            .map_err(|e| PaystackAPIError::Customer(e.to_string()))?;

        let response = self
            .http
            .post(&url, &self.key, &body)
            .await
            .map_err(|e| PaystackAPIError::Customer(e.to_string()))?;

        let parsed_response: Response<PhantomData<String>> = serde_json::from_str(&response)
            .map_err(|e| PaystackAPIError::Customer(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Whitelists or blacklists a customer on your integration
    ///
    /// # Arguments
    /// * `customer_code` - Customer's code or email address
    /// * `risk_action` - The risk action to apply to the customer
    ///
    /// # Returns
    /// A Result containing the updated customer response data or an error
    pub async fn whitelist_or_blacklist_customer(
        &self,
        customer_code: String,
        risk_action: RiskAction,
    ) -> PaystackResult<CustomerResponseData> {
        let url = format!("{}/set_risk_action", self.base_url);
        let body = json!({
            "customer": customer_code,
            "risk_action": risk_action
        });

        let response = self
            .http
            .post(&url, &self.key, &body)
            .await
            .map_err(|e| PaystackAPIError::Customer(e.to_string()))?;

        let parsed_response: Response<CustomerResponseData> = serde_json::from_str(&response)
            .map_err(|e| PaystackAPIError::Customer(e.to_string()))?;

        Ok(parsed_response)
    }

    /// Deactivates an authorization when the card needs to be forgotten
    ///
    /// # Arguments
    /// * `authorization_code` - Authorization code to be deactivated
    ///
    /// # Returns
    /// A Result containing the deactivation response or an error
    pub async fn deactivate_authorization(
        &self,
        authorization_code: String,
    ) -> PaystackResult<PhantomData<String>> {
        let url = format!("{}/authorization/deactivate", self.base_url);
        let body = json!({
            "authorization_code": authorization_code
        });

        let response = self
            .http
            .post(&url, &self.key, &body)
            .await
            .map_err(|e| PaystackAPIError::Customer(e.to_string()))?;

        let parsed_response: Response<PhantomData<String>> = serde_json::from_str(&response)
            .map_err(|e| PaystackAPIError::Customer(e.to_string()))?;

        Ok(parsed_response)
    }
}
