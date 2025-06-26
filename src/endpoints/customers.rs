//! Customers
//! =========
//! Thse Customers API allows you to create and maange customers on your integration

use std::sync::Arc;

use crate::{
    CreateCustomerRequest, CustomerResponseData, HttpClient, PaystackAPIError, PaystackResult,
    Response,
};

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
    /// Constructor
    pub fn new(key: Arc<String>, http: Arc<T>) -> CustomersEndpoints<T> {
        let base_url = String::from("https://api.paystack.co/customer");
        CustomersEndpoints {
            key: key.to_string(),
            base_url,
            http,
        }
    }

    /// Create customer on your integration
    ///
    /// It takes the following parameters:
    ///     - create_customer_request: contains the information about the customer to be created.
    ///     It should be built with `CreateCustomerRequestBuilder`.
    pub async fn create_customer(
        &self,
        create_customer_request: CreateCustomerRequest,
    ) -> PaystackResult<CustomerResponseData> {
        let url = format!("{}", self.base_url);
        let body = serde_json::to_value(create_customer_request)
            .map_err(|e| PaystackAPIError::Customer(e.to_string()))?;

        let response = self.http.post(&url, &self.key, &body).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<CustomerResponseData> =
                    serde_json::from_str(&response)
                        .map_err(|e| PaystackAPIError::Customer(e.to_string()))?;

                Ok(parsed_response)
            }
            Err(e) => Err(PaystackAPIError::Customer(e.to_string())),
        }
    }
}
