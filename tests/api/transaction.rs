use fake::faker::internet::en::SafeEmail;
use fake::Fake;
use paystack::{
    Channel, Currency, InitializeTransactionBodyBuilder, PartialDebitTransactionBodyBuilder, Status,
};
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
    let amount: String = rng.gen_range(100..=100000).to_string();
    let body = InitializeTransactionBodyBuilder::default()
        .amount(amount)
        .email(email)
        .currency(Some(Currency::NGN))
        .channels(Some(vec![
            Channel::ApplePay,
            Channel::BankTransfer,
            Channel::Bank,
        ]))
        .build()
        .unwrap();
    // println!("{:#?}", &body);
    let res = client
        .transaction
        .initialize_transaction(body)
        .await
        .expect("Unable to initialize transaction");

    // println!("{:#?}", res);

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
    let body = InitializeTransactionBodyBuilder::default()
        .amount(amount)
        .email(email)
        .currency(Some(Currency::USD))
        .channels(Some(vec![
            Channel::ApplePay,
            Channel::BankTransfer,
            Channel::Bank,
        ]))
        .build()
        .unwrap();

    let res = client.transaction.initialize_transaction(body).await;

    // Assert
    match res {
        Ok(_) => (),
        Err(e) => {
            let res = e.to_string();
            // dbg!("{:#?}", &res);
            assert!(res.contains("Status Code: 403 Forbidden"));
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
    let amount: String = rng.gen_range(100..=100000).to_string();
    let body = InitializeTransactionBodyBuilder::default()
        .amount(amount)
        .email(email)
        .currency(Some(Currency::NGN))
        .channels(Some(vec![
            Channel::ApplePay,
            Channel::BankTransfer,
            Channel::Bank,
        ]))
        .build()
        .unwrap();

    let content = client
        .transaction
        .initialize_transaction(body)
        .await
        .expect("unable to initiate transaction");

    let response = client
        .transaction
        .verify_transaction(&content.data.reference)
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
        .transaction
        .list_transactions(Some(5), Some(Status::Abandoned))
        .await
        .expect("unable to get list of integrated transactions");

    // Assert
    assert_eq!(5, response.data.len());
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
    assert_eq!(10, response.data.len());
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

    let fetched_transaction = client
        .transaction
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
        .transaction
        .list_transactions(Some(1), Some(Status::Success))
        .await
        .expect("unable to get list of integrated transactions");

    let transaction_timeline = client
        .transaction
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
        .transaction
        .list_transactions(Some(1), Some(Status::Success))
        .await
        .expect("unable to get list of integrated transactions");

    // println!("{:#?}", response);
    let reference = &response.data[0].reference.clone().unwrap();
    let transaction_timeline = client
        .transaction
        .view_transaction_timeline(None, Some(reference))
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
    let res = client
        .transaction
        .view_transaction_timeline(None, None)
        .await;

    // Assert
    match res {
        Ok(_) => (),
        Err(e) => {
            let res = e.to_string();
            assert!(
                res.contains("Transaction Id or Reference is need to view transaction timeline")
            );
        }
    }
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
    assert!(res.status);
    assert_eq!(res.message, "Transaction totals");
    assert!(res.data.total_transactions.is_some());
    assert!(res.data.total_volume.is_some());
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
    assert!(res.status);
    assert_eq!(res.message, "Export successful");
    assert!(!res.data.path.is_empty());
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

    let transaction = transaction.data[0].clone();
    let email = transaction.customer.unwrap().email.unwrap();
    let authorization_code = transaction
        .authorization
        .unwrap()
        .authorization_code
        .unwrap();
    let body = PartialDebitTransactionBodyBuilder::default()
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
            assert!(result.data.customer.unwrap().id.is_some())
        }
        Err(error) => {
            let error = error.to_string();
            assert!(error.contains("Status Code: 400 Bad Request"));
            assert!(error.contains("merchant is not enabled for Partial Debit"));
        }
    }
}
