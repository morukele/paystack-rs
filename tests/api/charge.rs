use crate::helpers::get_paystack_client;
use paystack::ChargeBuilder;
use rand::Rng;

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
