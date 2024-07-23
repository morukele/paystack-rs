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
//!

pub mod client;
pub mod endpoints;
pub mod errors;
pub mod http;
pub mod macros;
pub mod models;

// public re-export of modules
pub use client::*;
pub use endpoints::*;
pub use errors::*;
pub use http::*;
pub use macros::*;
pub use models::*;

/// Custom result type for the Paystack API
pub type PaystackResult<T> = Result<Response<T>, PaystackAPIError>;
