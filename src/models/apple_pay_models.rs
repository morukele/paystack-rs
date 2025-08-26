use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ApplePayResponseData {
    #[serde(rename = "domainNames")]
    pub domain_names: Vec<String>,
}
