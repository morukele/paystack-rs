//! Dedicated Virtual Account
//! =========================
//! The Dedicated Virtual Account API enables Nigerian and Ghanaian merchants to manage unique payment accounts of their customers.

use std::sync::Arc;

use crate::{http, HttpClient, PaystackResult};

#[derive(Debug, Clone)]
pub struct DedicatedVirtualAccountEndpoints<T: HttpClient + Default> {
    key: String,
    base_url: String,
    http: Arc<T>,
}

impl<T: HttpClient + Default> DedicatedVirtualAccountEndpoints<T> {
    pub fn new(key: Arc<String>, http: Arc<T>) -> DedicatedVirtualAccountEndpoints<T> {
        let base_url = String::from("https://api.paystack.co/dedicated_account");
        DedicatedVirtualAccountEndpoints {
            key: key.to_string(),
            base_url,
            http,
        }
    }

    /// Create a dedicated virtual account for an existing customer
    pub async fn create_dedicated_virtual_account(&self) {
        todo!()
    }
}
