//! Subaccounts
//! ===========
//! The Subaccounts API allows you create and manage subaccounts on your integration.
//! Subaccounts can be used to split payment between two accounts (your main account and a sub account).

use derive_builder::Builder;
use reqwest::StatusCode;
use serde::Serialize;

use crate::{post_request, CreateSubAccountResponse, Error, PaystackResult};

/// This struct is used to create the body for creating a subaccount on your integration.
#[derive(Serialize, Debug, Builder)]
pub struct CreateSubaccountBody {
    /// Name of business for subaccount
    business_name: String,
    /// Bank Code for the bank.
    /// You can get the list of Bank Codes by calling the List Banks endpoint.
    settlement_bank: String,
    /// Bank Account Number
    account_number: String,
    /// The default percentage charged when receiving on behalf of this subaccount
    percentage_charge: f32,
    /// A description for this subaccount
    description: String,
    /// A contact email for the subaccount
    #[builder(default = "None")]
    primary_contact_email: Option<String>,
    /// A name for the contact person for this subaccount
    #[builder(default = "None")]
    primary_contact_name: Option<String>,
    /// A phone number to call for this subaccount
    #[builder(default = "None")]
    primary_contact_phone: Option<String>,
    /// Stringified JSON object.
    /// Add a custom_fields attribute which has an array of objects if you would like the fields to be
    /// added to your transaction when displayed on the dashboard.
    /// Sample: {"custom_fields":[{"display_name":"Cart ID","variable_name": "cart_id","value": "8393"}]}
    #[builder(default = "None")]
    metadata: Option<String>,
}

/// A struct to hold all functions in the subaccount API route
#[derive(Debug, Clone)]
pub struct Subaccount {
    /// Paystack API key
    pub api_key: String,
}

static BASE_URL: &str = "https://api.paystack.co";

impl Subaccount {
    /// Creates a new subaccount.
    ///
    /// Takes in the following parameters
    ///     - body: subaccount to create.
    pub async fn create_subaccount(
        &self,
        body: CreateSubaccountBody,
    ) -> PaystackResult<CreateSubAccountResponse> {
        let url = format!("{}/subaccount", BASE_URL);

        match post_request(&self.api_key, &url, body).await {
            Ok(response) => match response.status() {
                StatusCode::CREATED => match response.json::<CreateSubAccountResponse>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(Error::Subaccount(err.to_string())),
                },
                _ => Err(Error::RequestNotSuccessful(
                    response.status().to_string(),
                    response.text().await?,
                )),
            },
            Err(err) => Err(Error::FailedRequest(err.to_string())),
        }
    }
}
