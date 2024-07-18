//! The HTTP client can be different and it is toggled during the configuration process of the crate
//! The default client will is th Reqwest client, in the case of none being selected.
//! If both are selected, a compiler error is raised.

pub mod base;
pub mod errors;
pub mod reqwest;

// public re-export
pub use base::HttpClient;
pub use errors::ReqwestError;
