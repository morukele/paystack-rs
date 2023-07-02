use paystack::PaystackClient;

fn get_api_key() -> String {
    let test_key = "sk_test_042c451f82e0ca5b3f305e650a6591f1dd8829c3";
    test_key.to_string()
}

pub fn get_paystack_client() -> PaystackClient {
    PaystackClient::new(get_api_key())
}
