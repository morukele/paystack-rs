#![deny(missing_docs)]

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
//!     paystack-rs = "0.1"
//! ```
//!
//! ## Usage
//!
//! Initializing an instance of the Paystack client and creating a transaction.
//!
//! ```rust
//!     use std::env;
//!     use dotenv::dotenv;
//!     use paystack::{PaystackClient, InitializeTransactionBodyBuilder, Error, Currency, Channel};
//!
//!     #[tokio::main]
//!     async fn main() -> Result<(), Error>{
//!         dotenv().ok();
//!         let api_key = env::var("PAYSTACK_API_KEY").unwrap();
//!         let client = PaystackClient::new(&api_key);
//!
//!         let body = InitializeTransactionBodyBuilder::default()
//!              .amount("10000".to_string())
//!              .email("email@example.com".to_string())
//!              .currency(Some(Currency::NGN))
//!              .channels(Some(vec![
//!                  Channel::ApplePay,
//!                  Channel::Bank,
//!                  Channel::BankTransfer
//!              ]))
//!              .build()
//!              .unwrap();
//!
//!        let transaction = client
//!             .transaction
//!             .initialize_transaction(body)
//!             .await
//!             .expect("Unable to create transaction");
//!
//!         Ok(())
//!     }
//!
//! ```
//!
//! ## Contributing
//!
//! See [CONTRIBUTING.md](/CONTRIBUTING.md) for information on contributing to paystack-rs.
//!
// ## License
//!
//! Licensed under MIT license ([LICENSE-MIT](/LICENSE-MIT)).

mod client;
mod endpoints;
mod error;
mod models;
mod response;
mod utils;

// public re-exports
pub use client::*;
pub use endpoints::*;
pub use error::*;
pub use models::*;
pub use response::*;
pub use utils::*;

/// Custom result type for the Paystack API
pub type PaystackResult<T> = std::result::Result<T, error::Error>;
