//! Subaccounts
//! ===========
//! The Subaccounts API allows you create and manage subaccounts on your integration.
//! Subaccounts can be used to split payment between two accounts (your main account and a sub account).

use derive_builder::Builder;
use reqwest::StatusCode;
use serde::Serialize;

use crate::{
    get_request, post_request, put_request, CreateSubaccountResponse, Error,
    FetchSubaccountResponse, ListSubaccountsResponse, PaystackResult,
};

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
    api_key: String,
}

static BASE_URL: &str = "https://api.paystack.co";

impl Subaccount {
    /// Constructor for the subaccount object
    pub fn new(key: String) -> Self {
        Subaccount { api_key: key }
    }

    /// Create a subaccount on your integration
    ///
    /// Takes in the following parameters
    ///     - body: subaccount to create.
    pub async fn create_subaccount(
        &self,
        body: CreateSubaccountBody,
    ) -> PaystackResult<CreateSubaccountResponse> {
        let url = format!("{}/subaccount", BASE_URL);

        match post_request(&self.api_key, &url, body).await {
            Ok(response) => match response.status() {
                StatusCode::CREATED => match response.json::<CreateSubaccountResponse>().await {
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

    /// List subaccounts available on your integration
    ///
    /// Take in the following parameters
    ///     - perPage: Specify how many records you want to retrieve per page. If not specify we use a default value of 50.
    ///     - page: Specify exactly what page you want to retrieve. If not specify we use a default value of 1.
    ///     - from: A timestamp from which to start listing subaccounts e.g. `2016-09-24T00:00:05.000Z`, `2016-09-21`
    ///     - to: A timestamp at which to stop listing subaccounts e.g. `2016-09-24T00:00:05.000Z`, `2016-09-21`
    pub async fn list_subaccounts(
        &self,
        per_page: Option<u32>,
        page: Option<u32>,
        from: Option<String>,
        to: Option<String>,
    ) -> PaystackResult<ListSubaccountsResponse> {
        let url = format!("{}/subaccount", BASE_URL);

        let query = vec![
            ("perPage", per_page.unwrap_or(50).to_string()),
            ("page", page.unwrap_or(1).to_string()),
            ("from", from.unwrap_or_default()),
            ("to", to.unwrap_or_default()),
        ];

        match get_request(&self.api_key, &url, Some(query)).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<ListSubaccountsResponse>().await {
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

    /// Get details of a subaccount on your integration.
    ///
    /// Takes the following parameters:
    ///     - id_or_code: The subaccount `ID` or `code` you want to fetch
    pub async fn fetch_subaccount(
        &self,
        id_or_code: String,
    ) -> PaystackResult<FetchSubaccountResponse> {
        let url = format!("{}/subaccount/{}", BASE_URL, id_or_code);

        match get_request(&self.api_key, &url, None).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<FetchSubaccountResponse>().await {
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

    /// Update a subaccount details on your integration.
    ///
    /// Takes the following parameters:
    ///     - id_or_code: Subaccount's ID or code.
    ///     - body: Subaccount modification payload
    pub async fn update_subaccount(
        &self,
        id_or_code: String,
        body: CreateSubaccountBody,
    ) -> PaystackResult<CreateSubaccountResponse> {
        let url = format!("{}/subaccount/{}", BASE_URL, id_or_code);

        match put_request(&self.api_key, &url, body).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<CreateSubaccountResponse>().await {
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
