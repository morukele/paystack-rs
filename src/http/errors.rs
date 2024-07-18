use thiserror::Error;

/// An error enum to hold errors from reqwest client
#[derive(Error, Debug)]
pub enum ReqwestError {
    /// Default HTTP error from the Reqwest crate.
    /// This happens when the request cannot be completed.
    #[error("request: {0}")]
    Reqwest(#[from] reqwest::Error),

    /// The initial request was successful, but the status code is in the 400
    /// and 500 range. This signifies that API cannot handle the request sent,
    /// We are only interested in the status code of this error
    #[error("status code: {}", reqwest::Response::status(.0))]
    StatusCode(reqwest::Response),
}
