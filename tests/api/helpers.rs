use std::env;

use dotenv::dotenv;
use paystack::PaystackClient;

fn get_api_key<'a>() -> &'a str {
    dotenv().ok();

    // Retrieve the environment variable as a String
    let key_str = env::var("PAYSTACK_API_KEY").expect("Unable to read API KEY from .env file.");

    // Convert the String to a string literal
    let key: &'a str = Box::leak(key_str.into_boxed_str());

    key
}

pub fn get_paystack_client<'a>() -> PaystackClient<'a> {
    PaystackClient::new(get_api_key())
}

pub fn get_bank_account_number_and_code() -> (String, String, String) {
    dotenv().ok();

    (
        env::var("BANK_ACCOUNT").expect("Unable to read Bank Account number from .env file."),
        env::var("BANK_CODE").expect("Unable to read Bank Code from .env file."),
        env::var("BANK_NAME").expect("Unable to read Bank Name from .env file."),
    )
}
