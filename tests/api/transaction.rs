use fake::faker::internet::en::SafeEmail;
use fake::Fake;
use paystack::{ChargeBuilder, TransactionBuilder};
use rand::Rng;

use crate::helpers::get_paystack_client;

#[tokio::test]
async fn initialize_transaction_valid() {
    // Arrange
    // This is a test key and is used for testing purposes only
    let client = get_paystack_client();
    let mut rng = rand::thread_rng();

    // Act
    let email: String = SafeEmail().fake();
    let body = TransactionBuilder::new()
        .amount(rng.gen_range(100..=100000).to_string())
        .email(email)
        .currency("NGN")
        .build()
        .unwrap();

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
    let email: String = SafeEmail().fake();
    let body = TransactionBuilder::new()
        .amount(rng.gen_range(100..=100000).to_string())
        .email(email)
        .currency("USD")
        .build()
        .unwrap();

    let res = client.initialize_transaction(body).await;

    // Assert
    match res {
        Ok(_) => (),
        Err(ex) => {
            let res = ex.to_string();
            assert!(res.contains("StatusCode: 403 Forbidden"));
            assert!(res.contains("Currency not supported by merchant"))
        }
    }
}

#[tokio::test]
async fn valid_transaction_is_verified() {
    // Arrange
    let client = get_paystack_client();
    let mut rng = rand::thread_rng();

    // Act
    let email: String = SafeEmail().fake();
    let body = TransactionBuilder::new()
        .amount(rng.gen_range(100..=100000).to_string())
        .email(email)
        .currency("NGN")
        .build()
        .unwrap();

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
        .list_transactions(Some(5), Some("success".to_string()))
        .await
        .expect("unable to get list of integrated transactions");

    println!("{:#?}", response);

    // Assert
    assert_eq!(5, response.data.len());
    assert!(response.status);
    assert_eq!("Transactions retrieved", response.message);
}

#[tokio::test]
async fn fetch_transaction_succeeds() {
    // Arrange
    let client = get_paystack_client();
    let mut rng = rand::thread_rng();

    // Act
    let email: String = SafeEmail().fake();
    let body = TransactionBuilder::new()
        .amount(rng.gen_range(100..=100000).to_string())
        .email(email)
        .currency("NGN")
        .build()
        .unwrap();

    let transaction = client
        .initialize_transaction(body)
        .await
        .expect("unable to initiate transaction");

    let verified_transaction = client
        .verify_transaction(transaction.data.reference.clone())
        .await
        .expect("unable to verify transaction");

    let fetched_transaction = client
        .fetch_transactions(verified_transaction.data.id.unwrap())
        .await
        .expect("unable to fetch transaction");

    // Assert
    assert_eq!(verified_transaction.data.id, fetched_transaction.data.id);
    assert_eq!(
        transaction.data.reference.clone(),
        fetched_transaction.data.reference.unwrap()
    );
}

#[tokio::test]
async fn charge_authorization_succeeds() {
    // Arrange
    let client = get_paystack_client();
    let mut rng = rand::thread_rng();

    // Act
    // In this test, an already created customer in the integration is used
    let charge = ChargeBuilder::new()
        .amount(rng.gen_range(100..=100000).to_string())
        .email("melyssa@example.net")
        .authorization_code("AUTH_9v3686msvt")
        .currency("NGN")
        .channel(vec!["card".to_string()])
        .transaction_charge(100)
        .build()
        .unwrap();

    let charge_response = client
        .charge_authorization(charge)
        .await
        .expect("unable to authorize charge");

    // Assert
    assert!(charge_response.status);
    assert_eq!(
        charge_response.data.customer.unwrap().email.unwrap(),
        "melyssa@example.net"
    );
    assert_eq!(
        charge_response
            .data
            .authorization
            .clone()
            .unwrap()
            .channel
            .unwrap(),
        "card"
    );
    assert_eq!(
        charge_response
            .data
            .authorization
            .unwrap()
            .authorization_code
            .unwrap(),
        "AUTH_9v3686msvt"
    );
}
