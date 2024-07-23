use dotenv::dotenv;
use paystack::http::reqwest::ReqwestClient;
use paystack::PaystackClient;
use std::env;

/// A function to get the bank information for the Paystack API
pub fn get_bank_account_number_and_code() -> (String, String, String) {
    dotenv().ok();

    (
        env::var("BANK_ACCOUNT").expect("Unable to read Bank Account number from .env file."),
        env::var("BANK_CODE").expect("Unable to read Bank Code from .env file."),
        env::var("BANK_NAME").expect("Unable to read Bank Name from .env file."),
    )
}

/// A function to get the base URL for the Paystack API
pub fn get_base_url() -> String {
    dotenv().ok();

    env::var("BASE_URL").unwrap_or(String::from("https://api.paystack.co"))
}

/// A function to get the API key
pub fn get_api_key() -> String {
    dotenv().ok();

    env::var("PAYSTACK_API_KEY").expect("Unable to read the paystack API key from .env file.")
}

/// A function to get an instance of the paystack client for testing
pub fn get_paystack_client() -> PaystackClient<ReqwestClient> {
    let base_url = get_base_url();
    let api_key = String::from("Fake key");

    PaystackClient::<ReqwestClient>::new(base_url, api_key)
}
