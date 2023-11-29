use crate::helpers::get_paystack_client;
use paystack::{Channel, ChargeBodyBuilder, Currency};
use rand::Rng;

/// Values are hardcoded in this test because of the nature of the test.
/// The values reflect the values in my integration.
/// If you can come up with a way to improve this test, take a stab at it.
#[tokio::test]
async fn charge_authorization_succeeds() {
    // Arrange
    let client = get_paystack_client();
    let mut rng = rand::thread_rng();

    // Act
    // In this test, an already created customer in the integration is used
    let amount = rng.gen_range(100..=100000).to_string();
    let charge = ChargeBodyBuilder::default()
        .email("susanna@example.net".to_string())
        .amount(amount)
        .authorization_code("AUTH_ik4t69fo2y".to_string())
        .currency(Some(Currency::NGN))
        .channel(Some(vec![Channel::Card]))
        .transaction_charge(Some(100))
        .build()
        .unwrap();

    let charge_response = client
        .transaction
        .charge_authorization(charge)
        .await
        .expect("unable to authorize charge");

    // Assert
    assert!(charge_response.status);
    assert_eq!(
        charge_response.data.customer.unwrap().email.unwrap(),
        "susanna@example.net"
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
        "AUTH_ik4t69fo2y"
    );
}
