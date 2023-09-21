//! Transaction Split
//! =================
//! The Transaction Splits API enables merchants split the settlement for a transaction
//! across their payout account, and one or more subaccounts.

use crate::{
    get_request, post_request, put_request, BearerType, Currency, Error, PaystackResult,
    ResponseWithoutData, SplitType, TransactionSplitListResponse, TransactionSplitResponse,
};
use derive_builder::Builder;
use reqwest::StatusCode;
use serde::Serialize;

/// This struct is used to create a split payment on your integration.
/// The struct is constructed using the `CreateTransactionSplitBodyBuilder`
#[derive(Serialize, Debug, Default, Builder)]
pub struct CreateTransactionSplitBody {
    /// Name of the transaction split
    name: String,
    /// The type of transaction split you want to create
    #[serde(rename = "type")]
    split_type: SplitType,
    /// Any of the supported currency
    currency: Currency,
    /// A list of object containing subaccount code and number of shares: `[{subaccount: ‘ACT_xxxxxxxxxx’, share: xxx},{...}]`
    subaccounts: Vec<SubaccountBody>,
    /// Any of subaccount
    bearer_type: BearerType,
    /// Subaccount code
    bearer_subaccount: String,
}

/// This struct represents the subaccount.
/// It can be used as the payload for the API end points that require a subaccount as a payload.
/// It is also possible to extract a single field from this struct to use as well.
/// The Struct is constructed using the `SubaccountBuilder`
#[derive(Serialize, Debug, Clone, Builder)]
pub struct SubaccountBody {
    /// This is the sub account code
    pub subaccount: String,
    /// This is the transaction share for the subaccount
    pub share: u32,
}

/// This struct is used to update a transaction split details on your integration.
/// The struct is constructed using the `UpdateTransactionSplitBodyBuilder`
#[derive(Serialize, Debug, Builder)]
pub struct UpdateTransactionSplitBody {
    /// Name of the transaction split
    pub name: String,
    /// True or False
    pub active: bool,
    /// Any of subaccount
    #[builder(default = "None")]
    pub bearer_type: Option<BearerType>,
    /// Subaccount code of a subaccount in the split group. This should be specified only if the `bearer_type is subaccount
    #[builder(default = "None")]
    pub bearer_subaccount: Option<SubaccountBody>,
}

/// A struct to hold all the functions of the transaction split API route
#[derive(Debug, Clone)]
pub struct TransactionSplit<'a> {
    /// Paystack API key
    api_key: &'a str,
}

static BASE_URL: &str = "https://api.paystack.co";

impl<'a> TransactionSplit<'a> {
    /// Constructor for the Transaction Split object
    pub fn new(key: &'a str) -> TransactionSplit<'a> {
        TransactionSplit { api_key: key }
    }

    /// Create a split payment on your integration.
    ///
    /// This method takes a TransactionSplit object as a parameter.
    pub async fn create_transaction_split(
        &self,
        split_body: CreateTransactionSplitBody,
    ) -> PaystackResult<TransactionSplitResponse> {
        let url = format!("{}/split", BASE_URL);

        match post_request(self.api_key, &url, split_body).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<TransactionSplitResponse>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(Error::TransactionSplit(err.to_string())),
                },
                _ => Err(Error::RequestNotSuccessful(
                    response.status().to_string(),
                    response.text().await?,
                )),
            },
            Err(err) => Err(Error::FailedRequest(err.to_string())),
        }
    }

    /// List the transaction splits available on your integration
    ///
    /// Takes in the following parameters:
    ///     - `split_name`: (Optional) name of the split to retrieve.
    ///     - `split_active`: (Optional) status of the split to retrieve.
    pub async fn list_transaction_splits(
        &self,
        split_name: Option<&str>,
        split_active: Option<bool>,
    ) -> PaystackResult<TransactionSplitListResponse> {
        let url = format!("{}/split", BASE_URL);

        // Specify a default option for active splits
        let split_active = match split_active {
            Some(active) => active.to_string(),
            None => "".to_string(),
        };

        let query = vec![
            ("name", split_name.unwrap_or("")),
            ("active", &split_active),
        ];

        match get_request(self.api_key, &url, Some(query)).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<TransactionSplitListResponse>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(Error::TransactionSplit(err.to_string())),
                },
                _ => Err(Error::RequestNotSuccessful(
                    response.status().to_string(),
                    response.text().await?,
                )),
            },
            Err(err) => Err(Error::FailedRequest(err.to_string())),
        }
    }

    /// Get details of a split on your integration.
    ///
    /// Takes in the following parameter:
    ///     - `split_id`:  Id of the transaction split.
    pub async fn fetch_transaction_split(
        &self,
        split_id: &str,
    ) -> PaystackResult<TransactionSplitResponse> {
        let url = format!("{}/split{}", BASE_URL, split_id);

        match get_request(self.api_key, &url, None).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<TransactionSplitResponse>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(Error::TransactionSplit(err.to_string())),
                },
                _ => Err(Error::RequestNotSuccessful(
                    response.status().to_string(),
                    response.text().await?,
                )),
            },
            Err(err) => Err(Error::FailedRequest(err.to_string())),
        }
    }

    /// Update a transaction split details on your integration.
    ///
    /// Takes in the following parameters:
    ///     - `split_id`: Id of the transaction split.
    ///     - `split_name`: updated name for the split.
    ///     - `split_status`: updated states for the split.
    ///     - `bearer_type`: (Optional) updated bearer type for the split.
    ///     - `bearer_subaccount`: (Optional) updated bearer subaccount for the split
    pub async fn update_transaction_split(
        &self,
        split_id: &str,
        body: UpdateTransactionSplitBody,
    ) -> PaystackResult<TransactionSplitResponse> {
        let url = format!("{}/split/{}", BASE_URL, split_id);

        match put_request(self.api_key, &url, body).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<TransactionSplitResponse>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(Error::TransactionSplit(err.to_string())),
                },
                _ => Err(Error::RequestNotSuccessful(
                    response.status().to_string(),
                    response.text().await?,
                )),
            },
            Err(err) => Err(Error::FailedRequest(err.to_string())),
        }
    }

    /// Add a Subaccount to a Transaction Split, or update the share of an existing Subaccount in a Transaction Split
    ///
    /// Takes in the following parameters:
    ///     - `split_id`: Id of the transaction split to update.
    ///     - `body`: Subaccount to add to the transaction split.
    pub async fn add_or_update_subaccount_split(
        &self,
        split_id: &str,
        body: SubaccountBody,
    ) -> PaystackResult<TransactionSplitResponse> {
        let url = format!("{}/split/{}/subaccount/add", BASE_URL, split_id);

        match post_request(self.api_key, &url, body).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<TransactionSplitResponse>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(Error::TransactionSplit(err.to_string())),
                },
                _ => Err(Error::RequestNotSuccessful(
                    response.status().to_string(),
                    response.text().await?,
                )),
            },
            Err(err) => Err(Error::FailedRequest(err.to_string())),
        }
    }

    /// Remove a subaccount from a transaction split.
    ///
    /// Takes in the following parameters
    ///     - split_id: Id of the transaction split
    ///     - subaccount: subaccount code to remove
    pub async fn remove_subaccount_from_transaction_split(
        &self,
        split_id: &str,
        subaccount: &str,
    ) -> PaystackResult<ResponseWithoutData> {
        let url = format!("{}/split/{}/subaccount/remove", BASE_URL, split_id);

        match post_request(self.api_key, &url, subaccount).await {
            Ok(response) => match response.status() {
                StatusCode::OK => match response.json::<ResponseWithoutData>().await {
                    Ok(content) => Ok(content),
                    Err(err) => Err(Error::TransactionSplit(err.to_string())),
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
