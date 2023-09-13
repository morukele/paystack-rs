use std::env;

use dotenv::dotenv;
use paystack::PaystackClient;

fn get_api_key() -> String {
    dotenv().ok();

    env::var("PAYSTACK_API_KEY").unwrap()
}

pub fn get_paystack_client() -> PaystackClient {
    PaystackClient::new(&get_api_key())
}
