use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::Currency;

#[derive(Debug, Serialize, Deserialize, Clone, Builder, Default)]
pub struct VirtualTerminalRequestData {
    /// Name of the virtual terminal
    pub name: String,
    /// An array of objects containing the notification recipients for payments to the Virtual Terminal.
    /// Create with the `DestinationRequestDataBuilder` struct.
    pub destinations: Vec<DestinationRequest>,
    /// Stringified JSON object of custom data.
    /// Kindly check the Paystack API Metadata page for more information
    #[builder(setter(strip_option), default)]
    pub metadata: Option<String>,
    /// The transaction currency for the Virtual Terminal. Defaults to your integration currency
    #[builder(setter(strip_option), default)]
    pub currency: Option<Vec<Currency>>,
    /// An array of objects representing custom fields to display on the form.
    /// Create with `CustomFieldBuilder` struct.
    #[builder(setter(strip_option), default)]
    pub custom_field: Option<Vec<CustomField>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Builder, Default)]
pub struct DestinationRequest {
    /// The Whatsapp phone number to send notifications to.
    pub target: String,
    /// A descriptive label
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Builder, Default)]
pub struct CustomField {
    /// What will be displayed on the Virtual Terminal page
    pub display_name: String,
    /// Parameter for referencing the custom field programmatically
    pub variable_name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct VirtualTerminalResponseData {
    pub id: u64,
    pub name: String,
    pub integration: u64,
    pub domain: String,
    pub code: String,
    pub payment_methods: Option<Vec<String>>,
    pub active: bool,
    pub metadata: Option<String>,
    pub destinations: Option<Vec<DestinationResponse>>,
    pub currency: Option<String>,
    pub created_at: Option<String>,
}

#[derive(Debug, Deserialize, Clone, Serialize, Default)]
pub struct DestinationResponse {
    pub target: Option<String>,
    #[serde(rename = "type")]
    pub destination_type: Option<String>,
    pub name: Option<String>,
    pub created_at: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_virtual_terminal_request() {
        let destinations = vec![
            DestinationRequestBuilder::default()
                .target("Whatsapp-phone-number".to_string())
                .name("Test-name".to_string())
                .build()
                .expect("unable to build destination request"),
            DestinationRequestBuilder::default()
                .target("Whatsapp-phone-number-2".to_string())
                .name("Test-name-2".to_string())
                .build()
                .expect("unable to build destination request"),
        ];
        let currencies = vec![Currency::NGN, Currency::USD];
        let custom_field = vec![
            CustomFieldBuilder::default()
                .display_name("display-name".to_string())
                .variable_name("variable-name".to_string())
                .build()
                .expect("unable to build custome field"),
            CustomFieldBuilder::default()
                .display_name("display-name-2".to_string())
                .variable_name("variable-name-2".to_string())
                .build()
                .expect("unable to build custome field"),
        ];

        let request = VirtualTerminalRequestDataBuilder::default()
            .name("Some name".to_string())
            .destinations(destinations)
            .currency(currencies)
            .custom_field(custom_field)
            .build()
            .expect("unable to build virtual terminal request");

        assert_eq!(request.name, "Some name");
        assert!(request.destinations.len() > 0);
        assert!(request.currency.is_some());
        assert!(request.custom_field.is_some());
        assert!(request.metadata.is_none());
    }

    #[test]
    fn cannot_create_virtual_terminal_request_without_compulsory_field() {
        let request = VirtualTerminalRequestDataBuilder::default()
            .currency(vec![Currency::GHS, Currency::NGN])
            .build();

        assert!(request.is_err());
    }
}
