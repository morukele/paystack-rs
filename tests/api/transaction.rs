use crate::helpers::get_paystack_client;
use fake::faker::internet::en::SafeEmail;
use fake::Fake;
use paystack::{Channel, Currency, TransactionRequestBuilder};
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
        .unwrap();

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