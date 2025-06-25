use crate::helpers::get_paystack_client;
use paystack::{Channel, ChargeRequestBuilder, Currency};
use rand::Rng;
use std::error::Error;

/// Values are hardcoded in this test because of the nature of the test.
/// The values reflect the values in my integration.
/// If you can come up with a way to improve this test, take a stab at it.
#[tokio::test]
async fn charge_authorization_succeeds() -> Result<(), Box<dyn Error>> {
    // Arrange
    let client = get_paystack_client();
    let mut rng = rand::thread_rng();

    // Act
    // In this test, an already created customer in the integration is used
    let amount = rng.gen_range(100..=100000).to_string();
    let charge = ChargeRequestBuilder::default()
        .email("susanna@example.net".to_string())
        .amount(amount)
        .authorization_code("AUTH_ik4t69fo2y".to_string())
        .currency(Currency::NGN)
        .channel(vec![Channel::Card])
        .transaction_charge(100)
        .build()?;

    let charge_response = client.transaction.charge_authorization(charge).await?;

    // Assert
    let data = charge_response.data.unwrap();
    assert!(charge_response.status);
    assert_eq!(data.customer.email, "susanna@example.net");
    assert_eq!(data.authorization.clone().channel, Some("card".into()));
    assert_eq!(
        data.authorization.authorization_code,
        Some("AUTH_ik4t69fo2y".into())
    );

    Ok(())
}
