use fake::{
    faker::{
        internet::en::SafeEmail,
        name::{
            en::Name,
            en::{FirstName, LastName},
        },
        phone_number::fr_fr::PhoneNumber,
    },
    Fake,
};

use crate::helpers::get_paystack_client;
use paystack::{CreateCustomerRequest, CreateCustomerRequestBuilder};

#[tokio::test]
async fn create_valid_subscription() {
    // Arrange
    let client = get_paystack_client();

    // Act
    // 1. create customer
    let email: String = SafeEmail().fake();
    let first_name: String = FirstName().fake();
    let last_name: String = LastName().fake();
    let phone: String = PhoneNumber().fake();

    let customer_body = CreateCustomerRequestBuilder::default()
        .email(email)
        .first_name(first_name)
        .last_name(last_name)
        .phone(phone)
        .build()
        .unwrap();

    let customer = client
        .customers
        .create_customer(customer_body)
        .await
        .expect("unable to create customer");
    // 2. create plan
    let plan_name: String = Name().fake();
    let mut rng = rand::rng();
    

    // 3. create subscription

    // Assert
    // 1. check that status is true
    // 2. check response message
    // 3. check aspect of response data
}
