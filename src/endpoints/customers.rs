use crate::{get_request, post_request};

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
}
