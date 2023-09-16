use std::env;

use dotenv::dotenv;
use paystack::PaystackClient;

fn get_api_key() -> String {
    dotenv().ok();

    env::var("PAYSTACK_API_KEY").unwrap()
}

pub fn get_paystack_client() -> PaystackClient {
    PaystackClient::new(get_api_key())
}

pub fn get_bank_account_number_and_code() -> (String, String) {
    dotenv().ok();

    (env::var("BANK_ACCOUNT").unwrap(), env::var("BANK_CODE").unwrap())
}