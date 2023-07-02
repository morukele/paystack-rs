mod client;
mod error;
mod response;

// public re-exports

// mapping results to our custom error type
pub type PaystackResult<T> = std::result::Result<T, error::PaystackError>;
pub use client::{PaystackClient, TransactionBody};
pub use error::PaystackError;
pub use error::RequestNotSuccessful;
pub use response::TransactionResponse;
