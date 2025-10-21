//! Domain
//! ======
//! This file constians the domain options for the integration in the paystack API.

use std::fmt;

use serde::{Deserialize, Serialize};

/// An enum of options for the paystack integration domain
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "lowercase")]
pub enum Domain {
    /// Integration in the test environment
    // Defaulting to test here for less danger
    #[default]
    Test,
    /// Integration in the live environment
    Live,
}

impl fmt::Display for Domain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let domain = match self {
            Domain::Test => "test",
            Domain::Live => "live",
        };
        write!(f, "{domain}")
    }
}
