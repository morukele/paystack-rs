use crate::helpers::get_paystack_client;
use fake::faker::internet::en::SafeEmail;
use fake::Fake;
use paystack::{Channel, Currency, PartialDebitTransactionRequestBuilder, Status, TransactionRequestBuilder};
use rand::Rng;
use std::error::Error;

#[tokio::test]
async fn initialize_transaction_valid() -> Result<(), Box<dyn Error>> {
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
        .build()?;

    let res = client.transaction.initialize_transaction(body).await?;

    // Assert
    assert!(res.status);
    assert_eq!("Authorization URL created", res.message);

    Ok(())
}

#[tokio::test]
async fn initialize_transaction_fails_when_currency_is_not_supported_by_merchant(
) -> Result<(), Box<dyn Error>> {
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
        .build()?;

    let res = client.transaction.initialize_transaction(body).await;

    // Assert
    match res {
        Ok(_) => (),
        Err(e) => {
            let res = e.to_string();
            assert!(res.contains("status code: 403 Forbidden"));
        }
    }

    Ok(())
}

#[tokio::test]
async fn valid_transaction_is_verified() -> Result<(), Box<dyn Error>> {
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
        .verify_transaction(&content.data.reference)
        .await
        .expect("unable to verify transaction");

    // Assert
    assert!(response.status);
    assert_eq!(response.message, "Verification successful");
    assert!(response.data.status.is_some());

    Ok(())
}

#[tokio::test]
async fn list_specified_number_of_transactions_in_the_integration() -> Result<(), Box<dyn Error>> {
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

    Ok(())
}

#[tokio::test]
async fn list_transactions_passes_with_default_values() -> Result<(), Box<dyn Error>> {
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

    Ok(())
}

#[tokio::test]
async fn fetch_transaction_succeeds() -> Result<(), Box<dyn Error>> {
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

    Ok(())
}

#[tokio::test]
async fn view_transaction_timeline_passes_with_id() -> Result<(), Box<dyn Error>> {
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

    Ok(())
}

#[tokio::test]
async fn view_transaction_timeline_passes_with_reference() -> Result<(), Box<dyn Error>> {
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

    Ok(())
}

#[tokio::test]
async fn view_transaction_timeline_fails_without_id_or_reference() -> Result<(), Box<dyn Error>> {
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

    Ok(())
}

#[tokio::test]
async fn get_transaction_total_is_successful() -> Result<(), Box<dyn Error>> {
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

    Ok(())
}

#[tokio::test]
async fn export_transaction_succeeds_with_default_parameters() -> Result<(), Box<dyn Error>> {
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

    Ok(())
}

#[tokio::test]
async fn partial_debit_transaction_passes_or_fails_depending_on_merchant_status() -> Result<(), Box<dyn Error>>{
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
            assert!(result.data.customer.unwrap().id.is_some())
        }
        Err(error) => {
            let error = error.to_string();
            assert!(error.contains("status code: 400 Bad Request"));
        }
    }

    Ok(())
}
