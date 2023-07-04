//!Convenient rust bindings and types for the Paystakc HTTP API aiming to support the entire API surface. Not the case? Please open an issue. I update the definitions on a weekly basis.
//! ## Documentatio
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
//! Initalizing an instance of the Paystack client and creating a transaction.
//!
//! ```rust
//!     use std::env;
//!     use dotenv::dotenv;
//!     use paystack::{PaystackClient, TransactionBuilder, PaystackError};
//!
//!     #[tokio::main]
//!     async fn main() -> Result<(), PaystackError>{
//!         dotenv().ok();
//!         let api_key = env::var("PAYSTACK_API_KEY").unwrap();
//!         let client = PaystackClient::new(api_key);
//!
//!         let body = TransactionBuilder::new()
//!             .email("email@example.com")
//!             .amount("200000")
//!             .currency("NGN")
//!             .build()
//!             .unwrap();
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
//! ## License
//!
//! Licensed under MIT license ([LICENSE-MIT](/LICENSE-MIT)).
//!
//!

mod client;
mod error;
mod request;
mod response;

// public re-exports
pub use client::PaystackClient;
pub use error::{PaystackError, RequestNotSuccessful};
pub use request::{Charge, ChargeBuilder, Transaction, TransactionBuilder};
pub use response::{
    Customer, TransactionResponse, TransactionResponseData, TransactionStatus,
    TransactionStatusData, TransactionStatusList,
};

// mapping results to our custom error type
pub type PaystackResult<T> = std::result::Result<T, error::PaystackError>;
