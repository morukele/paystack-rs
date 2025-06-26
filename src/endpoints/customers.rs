//! Customers
//! =========
//! Thse Customers API allows you to create and maange customers on your integration

use std::sync::Arc;

use crate::{
    CreateCustomerRequest, CustomerResponseData, HttpClient, PaystackAPIError, PaystackResult,
    Response, UpdateCustomerRequest,
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

    /// List customers available on your integration
    ///
    /// It takes the following parameters:
    ///     - `per_page`: Specify how many records you want to retreive per page. If not specified, default value of 50.
    ///     - `page`: Specify exactly waht page you want to retreive. If not specified, default value of 1.
    pub async fn list_customers(
        &self,
        per_page: Option<u8>,
        page: Option<u8>,
    ) -> PaystackResult<Vec<CustomerResponseData>> {
        let url = format!("{}", self.base_url);

        let per_page = per_page.unwrap_or(50).to_string();
        let page = page.unwrap_or(1).to_string();
        let query = vec![("perPage", per_page.as_str()), ("page", page.as_str())];

        let response = self.http.get(&url, &self.key, Some(&query)).await;

        match response {
            Ok(response) => {
                let parsed_response: Response<Vec<CustomerResponseData>> =
                    serde_json::from_str(&response)
                        .map_err(|e| PaystackAPIError::Customer(e.to_string()))?;

                Ok(parsed_response)
            }
            Err(e) => Err(PaystackAPIError::Customer(e.to_string())),
        }
    }

    /// Get details of a customer on your integration.
    ///
    /// It takes the following parameters:
    ///     - `email_or_code`: An `email`or `customer code` for the customer you want to fetch.
    pub async fn fetch_customer(
        &self,
        email_or_code: String,
    ) -> PaystackResult<CustomerResponseData> {
        let url = format!("{}/{}", self.base_url, email_or_code);

        let response = self.http.get(&url, &self.key, None).await;

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

    /// Update a customer's details on your integration
    ///
    /// It takes the following parameters:
    ///     - `code`: The customer's code
    ///     - `update_customer_request`: The data to update the customer with.
    ///     It should be created with the `UpdateCustomerRequestBuilder` struct.
    pub async fn update_customer(
        &self,
        code: String,
        update_customer_request: UpdateCustomerRequest,
    ) -> PaystackResult<CustomerResponseData> {
        let url = format!("{}/{}", self.base_url, code);
        let body = serde_json::to_value(update_customer_request)
            .map_err(|e| PaystackAPIError::Customer(e.to_string()))?;

        let response = self.http.put(&url, &self.key, &body).await;

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
