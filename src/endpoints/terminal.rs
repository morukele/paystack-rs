//! Terminal
//! ========
//! The Terminal API allows you to build delightful in-person payment experiences.

use crate::{
    get_request, post_request, put_request, Error, FetchEventStatusResponse, FetchTerminalResponse,
    FetchTerminalStatusResponse, PaystackResult, SendEventBody, SendEventResponse,
    UpdateTerminalBody, UpdateTerminalResponse,
};
use reqwest::{Response, StatusCode};
use std::fmt::format;

/// A Struct to hold all the functions of the terminal API route
#[derive(Debug, Clone)]
pub struct TerminalEndpoints<'a> {
    api_key: &'a str,
}

static BASE_URL: &str = "https://api.paystack.co/terminal";

impl<'a> TerminalEndpoints<'a> {
    /// Constructor for the terminal object
    pub fn new(key: &'a str) -> TerminalEndpoints<'a> {
        TerminalEndpoints { api_key: key }
    }

    /// Send an event from your application to the Paystack Terminal
    ///
    /// It takes a SendEventBody type as its parameter
    pub async fn send_event(
        &self,
        terminal_id: &str,
        event_body: SendEventBody,
    ) -> PaystackResult<SendEventResponse> {
        let url = format!("{}/{}/event", BASE_URL, terminal_id);

        match post_request(self.api_key, &url, event_body).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<SendEventResponse>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(Error::Terminal(err.to_string())),
                },
                _ => Err(Error::RequestNotSuccessful(
                    response.status().to_string(),
                    response.text().await?,
                )),
            },
            Err(err) => Err(Error::FailedRequest(err.to_string())),
        }
    }

    /// Check the status of an event sent to the Terminal
    ///
    /// - terminal_id: The ID of the Terminal the event was sent to.
    /// - event_id: The ID of the event that was sent to the Terminal.
    pub async fn fetch_event_status(
        &self,
        terminal_id: &str,
        event_id: &str,
    ) -> PaystackResult<FetchEventStatusResponse> {
        let url = format!("{}/{}/event/{}", BASE_URL, terminal_id, event_id);

        match get_request(self.api_key, &url, None).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<FetchEventStatusResponse>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(Error::Terminal(err.to_string())),
                },
                _ => Err(Error::RequestNotSuccessful(
                    response.status().to_string(),
                    response.text().await?,
                )),
            },
            Err(err) => Err(Error::FailedRequest(err.to_string())),
        }
    }

    /// Check the availability of a Terminal before sending an event to it
    ///
    /// - terminal_id: The ID of the Terminal you want to check
    pub async fn fetch_terminal_status(
        &self,
        terminal_id: &str,
    ) -> PaystackResult<FetchTerminalStatusResponse> {
        let url = format!("{}/{}/presence", BASE_URL, terminal_id);

        match get_request(self.api_key, &url, None).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<FetchTerminalStatusResponse>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(Error::Terminal(err.to_string())),
                },
                _ => Err(Error::RequestNotSuccessful(
                    response.status().to_string(),
                    response.text().await?,
                )),
            },
            Err(err) => Err(Error::FailedRequest(err.to_string())),
        }
    }

    /// Get the details of a Terminal
    ///
    ///- terminal_id: The ID of the Terminal the event was sent to.
    pub async fn fetch_terminal(&self, terminal_id: &str) -> PaystackResult<FetchTerminalResponse> {
        let url = format!("{}/{}", BASE_URL, terminal_id);

        match get_request(self.api_key, &url, None).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<FetchTerminalResponse>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(Error::Terminal(err.to_string())),
                },
                _ => Err(Error::RequestNotSuccessful(
                    response.status().to_string(),
                    response.text().await?,
                )),
            },
            Err(err) => Err(Error::FailedRequest(err.to_string())),
        }
    }

    /// Update the details of a Terminal
    pub async fn update_terminal(
        &self,
        terminal_id: &str,
        update_terminal_body: UpdateTerminalBody,
    ) -> PaystackResult<UpdateTerminalResponse> {
        let url = format!("{}/{}", BASE_URL, terminal_id);

        match put_request(self.api_key, &url, update_terminal_body).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<UpdateTerminalResponse>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(Error::Terminal(err.to_string())),
                },
                _ => Err(Error::RequestNotSuccessful(
                    response.status().to_string(),
                    response.text().await?,
                )),
            },
            Err(err) => Err(Error::FailedRequest(err.to_string())),
        }
    }

    ///
    pub async fn commission_terminal() {}

    ///
    pub async fn decommission_terminal() {}
}
