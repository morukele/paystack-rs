use fake::faker::internet::en::SafeEmail;
use fake::Fake;
use paystack::TransactionBuilder;
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
        Err(e) => {
            let res = e.to_string();
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

    // Assert
    assert_eq!(5, response.data.len());
    assert!(response.status);
    assert_eq!("Transactions retrieved", response.message);
}

#[tokio::test]
async fn fetch_transaction_succeeds() {
    // Arrange
    let client = get_paystack_client();

    // Act
    let response = client
        .list_transactions(Some(1), Some("success".to_string()))
        .await
        .expect("unbale to get list of integrated transactions");

    let fetched_transaction = client
        .fetch_transactions(response.data[0].id.unwrap())
        .await
        .expect("unable to fetch transaction");

    // Assert
    assert_eq!(response.data[0].id, fetched_transaction.data.id);
    assert_eq!(
        response.data[0].reference,
        fetched_transaction.data.reference
    );
}

#[tokio::test]
async fn view_transaction_timeline_passes_with_id() {
    // Arrange
    let client = get_paystack_client();

    // Act
    let response = client
        .list_transactions(Some(1), Some("success".to_string()))
        .await
        .expect("unable to get list of integrated transactions");

    let transaction_timeline = client
        .view_transaction_timeline(response.data[0].id, None)
        .await
        .expect("unable to get transaction timeline");

    // Assert
    assert!(transaction_timeline.status);
    assert_eq!(transaction_timeline.message, "Timeline retrieved");
}

#[tokio::test]
async fn view_transaction_timeline_passes_with_reference() {
    // Arrange
    let client = get_paystack_client();

    // Act
    let response = client
        .list_transactions(Some(1), Some("success".to_string()))
        .await
        .expect("unable to get list of integrated transactions");

    let transaction_timeline = client
        .view_transaction_timeline(None, response.data[0].reference.clone())
        .await
        .expect("unable to get transaction timeline");

    // Assert
    assert!(transaction_timeline.status);
    assert_eq!(transaction_timeline.message, "Timeline retrieved");
}

#[tokio::test]
async fn view_transaction_timeline_fails_without_id_or_reference() {
    // Arrange
    let client = get_paystack_client();

    // Act
    let res = client.view_transaction_timeline(None, None).await;

    // Assert
    match res {
        Ok(_) => (),
        Err(e) => {
            let res = e.to_string();
            assert!(res.contains("StatusCode: 400"));
            assert!(res.contains("Transaction ID should be numeric"));
        }
    }
}
