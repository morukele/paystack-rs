//! Virtual Terminal
//! ================
//! The Virtual Terminal API allows you to accept in-person payments without a POS device.

use std::sync::Arc;

use crate::{HttpClient, PaystackResult};

#[derive(Debug, Clone)]
pub struct VirtualTerminalEndpoints<T: HttpClient + Default> {
    /// Paystack API key
    key: String,
    /// Base URL for the transaction route
    base_url: String,
    /// Http client for the route
    http: Arc<T>,
}

impl<T: HttpClient + Default> VirtualTerminalEndpoints<T> {
    /// Constructor
    pub fn new(key: Arc<String>, http: Arc<T>) -> VirtualTerminalEndpoints<T> {
        let base_url = String::from("https://api.paystack.co/virtual_terminal");
        VirtualTerminalEndpoints {
            key: key.to_string(),
            base_url,
            http,
        }
    }

    // pub async fn create_virtual_terminal() -> PaystackResult {}
}
