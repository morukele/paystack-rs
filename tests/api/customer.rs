use fake::{
    faker::{
        internet::en::SafeEmail,
        name::en::{FirstName, LastName, Name},
        phone_number::fr_fr::PhoneNumber,
    },
    Fake,
};
use paystack::CreateCustomerBodyBuilder;

use crate::helpers::get_paystack_client;

#[tokio::test]
async fn create_customer_with_good_data_is_valid() {
    // Arrange
    let client = get_paystack_client();

    // Act
    let email: String = SafeEmail().fake();
    let first_name: String = FirstName().fake();
    let last_name: String = LastName().fake();
    let phone: String = PhoneNumber().fake();

    let body = CreateCustomerBodyBuilder::default()
        .email(email)
        .first_name(first_name)
        .last_name(last_name)
        .phone(phone)
        .build()
        .unwrap();
    // println!("{:?}", &body);
    let res = client
        .customer
        .create_customer(body)
        .await
        .expect("Unable to create customer");
    // println!("{:#?}", res);

    // Assert
    assert!(res.status);
    assert_eq!("Customer created", res.message);
}

#[tokio::test]
async fn create_customer_with_bad_email_is_not_valid() {
    // Arrange
    let client = get_paystack_client();

    // Act
    let email: String = Name().fake();
    let first_name: String = FirstName().fake();
    let last_name: String = LastName().fake();
    let phone: String = Name().fake();

    let body = CreateCustomerBodyBuilder::default()
        .email(email)
        .first_name(first_name)
        .last_name(last_name)
        .phone(phone)
        .build()
        .unwrap();

    let res = client.customer.create_customer(body).await;

    // Assert
    match res {
        Ok(_) => (),
        Err(e) => {
            let res = e.to_string();
            //dbg!("{:#?}", &res);
            assert!(res.contains("Status Code: 400 Bad Request Body"));
            assert!(res.contains("must be a valid email"));
        }
    }
}
