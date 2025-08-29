//! Convenient rust bindings and types for the Paystack HTTP API aiming to support the entire API surface.
//! Not the case? Please open an issue. I update the definitions on a weekly basis.
//!
//! # Documentation
//! See the [Rust API docs](https://docs.rs/paystack-rs) or the [examples](/examples).
//!
//! ## Installation
//!
//! `paystack-rs` uses the `reqwest` http client under the hood and the `tokio` runtime for async operations
//!
//! ```toml
//!     [dependencies]
//!     paystack-rs = "1.5.0"
//! ```
//!
//! ## Usage
//!
//! Initializing an instance of the Paystack client and creating a transaction.
//!
//! ```rust
//! use std::env;
//! use std::error::Error;
//! use dotenv::dotenv;
//! use paystack::{PaystackClient, TransactionRequestBuilder, PaystackAPIError, Currency, Channel, ReqwestClient};
//!
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     dotenv().ok();
//!     use std::error::Error;
//!     let api_key = env::var("PAYSTACK_API_KEY").unwrap();
//!     let client = PaystackClient::<ReqwestClient>::new(api_key);
//!
//!
//!     let email = "email@example.com".to_string();
//!     let amount ="10000".to_string();
//!     let body = TransactionRequestBuilder::default()
//!         .amount(amount)
//!         .email(email)
//!         .currency(Currency::NGN)
//!         .channel(vec![
//!             Channel::Card,
//!             Channel::ApplePay,
//!             Channel::BankTransfer,
//!             Channel::Bank,
//!         ])
//!         .build()?;
//!
//!     let res = client
//!         .transactions
//!         .initialize_transaction(body)
//!         .await
//!         .expect("Unable to create transaction");
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Contributing
//!
//! See [CONTRIBUTING.md](/CONTRIBUTING.md) for information on contributing to paystack-rs.
//!
// ## License
//!
//! Licensed under MIT license ([LICENSE-MIT](/LICENSE-MIT)).
//!

pub mod client;
pub mod endpoints;
pub mod errors;
pub mod http;
pub mod macros;
pub mod models;
pub mod utils;

// public re-export of modules
pub use client::*;
pub use endpoints::*;
pub use errors::*;
pub use http::*;
pub use models::*;
pub use utils::*;

/// Custom result type for the Paystack API
pub type PaystackResult<T> = Result<Response<T>, PaystackAPIError>;
