use fake::{
    faker::{
        internet::en::SafeEmail,
        name::en::{FirstName, LastName},
        phone_number::fr_fr::PhoneNumber,
    },
    Fake,
};
use paystack::CreateCustomerRequestBuilder;

use crate::helpers::get_paystack_client;

#[tokio::test]
async fn create_customer_with_valid_data_succeed() {
    // Arrange
    let client = get_paystack_client();

    // Act
    let email: String = SafeEmail().fake();
    let first_name: String = FirstName().fake();
    let last_name: String = LastName().fake();
    let phone: String = PhoneNumber().fake();

    let body = CreateCustomerRequestBuilder::default()
        .email(email.clone())
        .first_name(first_name)
        .last_name(last_name)
        .phone(phone)
        .build()
        .unwrap();

    let res = client
        .customers
        .create_customer(body)
        .await
        .expect("unable to create customer");

    // Assert
    let data = res.data.unwrap();
    assert!(res.status);
    assert_eq!(&data.email, &email);
    assert!(!&data.identified.unwrap());
}
