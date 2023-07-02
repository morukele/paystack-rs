use fake::faker::internet::en::SafeEmail;
use fake::Fake;
use paystack::TransactionBody;
use rand::Rng;

use crate::helpers::get_paystack_client;

#[tokio::test]
async fn initialize_transaction_valid() {
    // Arrange
    // This is a test key and is used for testing purposes only
    let client = get_paystack_client();
    let mut rng = rand::thread_rng();

    // Act
    let body = TransactionBody {
        amount: rng.gen_range(100..=100000).to_string(),
        email: SafeEmail().fake(),
        currency: None,
    };

    let res = client
        .initialize_transaction(body)
        .await
        .expect("Unable to initalize transaction");

    // Assert
    assert!(res.status);
    assert_eq!("Authorization URL created", res.message);
}

#[tokio::test]
async fn initialize_transaction_fails_when_currency_is_not_supported_by_marchent() {
    // Arrange
    let client = get_paystack_client();
    let mut rng = rand::thread_rng();

    // Act
    let body = TransactionBody {
        amount: rng.gen_range(100..=100000).to_string(),
        email: SafeEmail().fake(),
        currency: Some("USD".to_string()),
    };

    let res = client.initialize_transaction(body).await.err();
    let res = res.unwrap().to_string();

    // Assert
    assert!(res.contains("StatusCode: 403 Forbidden"));
    assert!(res.contains("Currency not supported by merchant"))
}

#[tokio::test]
async fn valid_transaction_is_verified() {
    // Arrange
    let client = get_paystack_client();
    let mut rng = rand::thread_rng();

    // Act
    let body = TransactionBody {
        amount: rng.gen_range(100..=100000).to_string(),
        email: SafeEmail().fake(),
        currency: Some("NGN".to_string()),
    };
    let content = client
        .initialize_transaction(body)
        .await
        .expect("unable to initiate transaction");

    let response = client
        .verify_transaction(content.data.reference)
        .await
        .expect("unable to verify transaction");

    // Assert
    assert!(response.status);
    assert_eq!(response.message, "Verification successful");
    assert!(response.data.status.is_some());
}

#[tokio::test]
async fn list_specified_number_of_transactions_in_the_integration() {
    // Arrange
    let client = get_paystack_client();

    // Act
    let response = client
        .list_transactions(Some(5))
        .await
        .expect("unable to get list of integrated transactions");

    // Assert
    assert_eq!(5, response.data.len());
    assert!(response.status);
    assert_eq!("Transactions retrieved", response.message);
}
