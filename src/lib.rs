#![deny(missing_docs)]

//! Convenient rust bindings and types for the Paystakc HTTP API aiming to support the entire API surface.
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
//!     use paystack::{PaystackClient, InitializeTransactionBody, Error, Currency, Channel};
//!
//!     #[tokio::main]
//!     async fn main() -> Result<(), Error>{
//!         dotenv().ok();
//!         let api_key = env::var("PAYSTACK_API_KEY").unwrap();
//!         let client = PaystackClient::new(api_key);
//!
//!         let body = InitializeTransactionBody {
//!             amount: "20000".to_string(),
//!             email: "email@example.com".to_string(),
//!             currency: Some(Currency::NGN),
//!             channels: Some(vec![
//!                 Channel::ApplePay,
//!                 Channel::BankTransfer,
//!                 Channel::Bank,
//!             ]),
//!             bearer: None,
//!             callback_url: None,
//!             invoice_limit: None,
//!             metadata: None,
//!             plan: None,
//!             reference: None,
//!             split_code: None,
//!             subaccount: None,
//!             transaction_charge: None,
//!         };
//!
//!         let transaction = client
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
mod error;
mod resources;
mod response;
mod utils;

// public re-exports
pub use client::*;
pub use error::*;
pub use resources::*;
pub use response::*;
pub use utils::*;

/// Custom result type for the Paystack API
pub type PaystackResult<T> = std::result::Result<T, error::Error>;
