use reqwest::StatusCode;

use crate::{post_request, CreateCustomerBody, CreateCustomerResponse, Error, PaystackResult, ListCustomerResponse};

/// A struct to hold all the functions of the customer API route
#[derive(Debug, Clone)]
pub struct CustomerEndpoints<'a> {
    /// Paystack API key
    api_key: &'a str,
}

static BASE_URL: &str = "https://api.paystack.co/customer";

impl<'a> CustomerEndpoints<'a> {
    /// Constructor for the customer object
    pub fn new(key: &'a str) -> CustomerEndpoints<'a> {
        CustomerEndpoints { api_key: key }
    }

    /// Create a customer on your integration
    /// 
    /// It takes a CreateCustomerBody as its parameter
    pub async fn create_customer(
        self,
        body: CreateCustomerBody,
    ) -> PaystackResult<CreateCustomerResponse> {
        let url = BASE_URL.to_string();

        match post_request(self.api_key, &url, body).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<CreateCustomerResponse>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(Error::Customer(err.to_string())),
                },
                _ => Err(Error::RequestNotSuccessful(
                    response.status().to_string(),
                    response.text().await?,
                )),
            },
            Err(err) => Err(Error::FailedRequest(err.to_string())),
        }
    }

    /// List the customers in the integration
    pub async fn list_customers() -> PaystackResult<ListCustomerResponse> {

    }
}
