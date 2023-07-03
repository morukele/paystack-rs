mod client;
mod error;
mod request;
mod response;

// public re-exports
pub use client::PaystackClient;
pub use error::{PaystackError, RequestNotSuccessful};
pub use request::{Transaction, TransactionBuilder};
pub use response::{
    Customer, TransactionResponse, TransactionResponseData, TransactionStatus,
    TransactionStatusData, TransactionStatusList,
};

// mapping results to our custom error type
pub type PaystackResult<T> = std::result::Result<T, error::PaystackError>;
