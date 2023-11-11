//! Subaccounts
//! ===========
//! The Subaccounts API allows you create and manage subaccounts on your integration.
//! Subaccounts can be used to split payment between two accounts (your main account and a sub account).

use reqwest::StatusCode;

use crate::{
    get_request, post_request, put_request, CreateSubaccountBody, CreateSubaccountResponse, Error,
    FetchSubaccountResponse, ListSubaccountsResponse, PaystackResult,
};

/// A struct to hold all functions in the subaccount API route
#[derive(Debug, Clone)]
pub struct SubaccountEndpoints<'a> {
    /// Paystack API key
    api_key: &'a str,
}

static BASE_URL: &str = "https://api.paystack.co/subaccount";

impl<'a> SubaccountEndpoints<'a> {
    /// Constructor for the subaccount object
    pub fn new(key: &'a str) -> SubaccountEndpoints<'a> {
        SubaccountEndpoints { api_key: key }
    }

    /// Create a subaccount on your integration
    ///
    /// Takes in the following parameters
    ///     - body: subaccount to create.
    pub async fn create_subaccount(
        &self,
        body: CreateSubaccountBody,
    ) -> PaystackResult<CreateSubaccountResponse> {
        let url = format!("{}", BASE_URL);

        match post_request(self.api_key, &url, body).await {
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
        from: Option<&str>,
        to: Option<&str>,
    ) -> PaystackResult<ListSubaccountsResponse> {
        let url = format!("{}", BASE_URL);

        let per_page = per_page.unwrap_or(50).to_string();
        let page = page.unwrap_or(1).to_string();

        let query = vec![
            ("perPage", per_page.as_str()),
            ("page", page.as_str()),
            ("from", from.unwrap_or_default()),
            ("to", to.unwrap_or_default()),
        ];

        match get_request(self.api_key, &url, Some(query)).await {
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
        id_or_code: &str,
    ) -> PaystackResult<FetchSubaccountResponse> {
        let url = format!("{}/{}", BASE_URL, id_or_code);

        match get_request(self.api_key, &url, None).await {
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
        id_or_code: &str,
        body: CreateSubaccountBody,
    ) -> PaystackResult<CreateSubaccountResponse> {
        let url = format!("{}/{}", BASE_URL, id_or_code);

        match put_request(self.api_key, &url, body).await {
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
