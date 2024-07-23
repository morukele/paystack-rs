use crate::helpers::get_paystack_client;
use fake::faker::internet::en::SafeEmail;
use fake::Fake;
use paystack::{Channel, Currency, Status, TransactionRequestBuilder};
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
async fn list_specified_number_of_transactions_in_the_integration() -> Result<(), Box<dyn Error>>{
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
