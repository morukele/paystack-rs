use crate::helpers::get_paystack_client;
use fake::faker::internet::en::SafeEmail;
use fake::Fake;
use paystack::{
    Channel, Currency, PartialDebitTransactionRequestBuilder, Status, TransactionIdentifier,
    TransactionRequestBuilder,
};
use rand::Rng;

#[tokio::test]
async fn initialize_transaction_valid() {
    // Arrange
    let client = get_paystack_client();
    let mut rng = rand::thread_rng();

    // Act
    let email: String = SafeEmail().fake();
    let amount: String = rng.gen_range(100..=10_000).to_string();
    let body = TransactionRequestBuilder::default()
        .amount(amount)
        .email(email)
        .currency(Currency::NGN)
        .channel(vec![
            Channel::Card,
            Channel::ApplePay,
            Channel::BankTransfer,
            Channel::Bank,
        ])
        .build()
        .unwrap();

    let res = client
        .transaction
        .initialize_transaction(body)
        .await
        .expect("unable to create transaction");

    // Assert
    assert!(res.status);
    assert_eq!("Authorization URL created", res.message);
}

#[tokio::test]
async fn initialize_transaction_fails_when_currency_is_not_supported_by_merchant() {
    // Arrange
    let client = get_paystack_client();
    let mut rng = rand::thread_rng();

    // Act
    let email: String = SafeEmail().fake();
    let amount: String = rng.gen_range(100..=100000).to_string();
    let body = TransactionRequestBuilder::default()
        .amount(amount)
        .email(email)
        .currency(Currency::GHS)
        .channel(vec![
            Channel::ApplePay,
            Channel::BankTransfer,
            Channel::Bank,
        ])
        .build()
        .expect("unable to build Transaction Request");

    let res = client.transaction.initialize_transaction(body).await;

    // Assert
    match res {
        Ok(_) => (),
        Err(e) => {
            let res = e.to_string();
            assert!(res.contains("status code: 403 Forbidden"));
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
    let amount: String = rng.gen_range(100..=100000).to_string();
    let body = TransactionRequestBuilder::default()
        .amount(amount)
        .email(email)
        .currency(Currency::NGN)
        .channel(vec![
            Channel::ApplePay,
            Channel::BankTransfer,
            Channel::Bank,
        ])
        .build()
        .unwrap();

    let content = client
        .transaction
        .initialize_transaction(body)
        .await
        .expect("unable to initiate transaction");

    let response = client
        .transaction
        .verify_transaction(&content.data.unwrap().reference)
        .await
        .expect("unable to verify transaction");

    // Assert
    assert!(response.status);
    assert_eq!(response.message, "Verification successful");
    assert_eq!(response.data.unwrap().status, "abandoned");
}

#[tokio::test]
async fn list_specified_number_of_transactions_in_the_integration() {
    // Arrange
    let client = get_paystack_client();

    // Act
    let response = client
        .transaction
        .list_transactions(Some(5), Some(Status::Abandoned))
        .await
        .expect("unable to get list of integrated transactions");

    // Assert
    assert_eq!(5, response.data.unwrap().len());
    assert!(response.status);
    assert_eq!("Transactions retrieved", response.message);
}

#[tokio::test]
async fn list_transactions_passes_with_default_values() {
    // Arrange
    let client = get_paystack_client();

    // Act
    let response = client
        .transaction
        .list_transactions(None, None)
        .await
        .expect("unable to get list of integration transactions");

    // Assert
    assert!(response.status);
    assert_eq!(10, response.data.unwrap().len());
    assert_eq!("Transactions retrieved", response.message);
}

#[tokio::test]
async fn fetch_transaction_succeeds() {
    // Arrange
    let client = get_paystack_client();

    // Act
    let response = client
        .transaction
        .list_transactions(Some(1), Some(Status::Success))
        .await
        .expect("unable to get list of integrated transactions");

    let data = response.data.unwrap();
    let fetched_transaction = client
        .transaction
        .fetch_transactions(data[0].id)
        .await
        .expect("unable to fetch transaction");

    // Assert
    let res = fetched_transaction.data.unwrap();
    assert_eq!(&data[0].id, &res.id);
    assert_eq!(&data[0].reference, &res.reference);
}

#[tokio::test]
async fn view_transaction_timeline_passes_with_id() {
    // Arrange
    let client = get_paystack_client();

    // Act
    let response = client
        .transaction
        .list_transactions(Some(1), Some(Status::Success))
        .await
        .expect("unable to get list of integrated transactions");

    let data = response.data.unwrap();
    let identifier = TransactionIdentifier::Id(data[0].id);

    let transaction_timeline = client
        .transaction
        .view_transaction_timeline(identifier)
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
        .transaction
        .list_transactions(Some(1), Some(Status::Success))
        .await
        .expect("unable to get list of integrated transactions");

    // println!("{:#?}", response);
    let data = response.data.unwrap();
    let reference = data[0].reference.clone();
    let identifier = TransactionIdentifier::Reference(reference);
    let transaction_timeline = client
        .transaction
        .view_transaction_timeline(identifier)
        .await
        .expect("unable to get transaction timeline");

    // Assert
    assert!(transaction_timeline.status);
    assert_eq!(transaction_timeline.message, "Timeline retrieved");
}

#[tokio::test]
async fn get_transaction_total_is_successful() {
    // Arrange
    let client = get_paystack_client();

    // Act
    let res = client
        .transaction
        .total_transactions()
        .await
        .expect("unable to get transaction total");

    // Assert
    let data = res.data.unwrap();
    assert!(res.status);
    assert_eq!(res.message, "Transaction totals");
    assert!(data.total_transactions.is_some());
    assert!(data.total_volume.is_some());
}

#[tokio::test]
async fn export_transaction_succeeds_with_default_parameters() {
    // Arrange
    let client = get_paystack_client();

    // Act
    let res = client
        .transaction
        .export_transaction(None, None, None)
        .await
        .expect("unable to export transactions");

    // Assert
    let data = res.data.unwrap();
    assert!(res.status);
    assert_eq!(res.message, "Export successful");
    assert!(!data.path.is_empty());
}

#[tokio::test]
async fn partial_debit_transaction_passes_or_fails_depending_on_merchant_status() {
    // Arrange
    let client = get_paystack_client();

    // Act
    let transaction = client
        .transaction
        .list_transactions(Some(1), Some(Status::Success))
        .await
        .expect("Unable to get transaction list");

    let data = transaction.data.unwrap();
    let transaction = data[0].clone();
    let email = transaction.customer.email;
    let authorization_code = transaction.authorization.authorization_code.unwrap();
    let body = PartialDebitTransactionRequestBuilder::default()
        .email(email)
        .amount("10000".to_string())
        .authorization_code(authorization_code)
        .currency(Currency::NGN)
        .build()
        .unwrap();

    let res = client.transaction.partial_debit(body).await;

    // Assert
    match res {
        Ok(result) => {
            assert!(result.status);
            assert_eq!(result.message, "Charge attempted");
        }
        Err(error) => {
            let error = error.to_string();
            assert!(error.contains("status code: 400 Bad Request"));
        }
    }
}
