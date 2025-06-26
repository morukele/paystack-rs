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

#[tokio::test]
async fn can_list_customers_in_integration() {
    // Arrange
    let clinet = get_paystack_client();

    // Act
    let per_page = 10;
    let page = 1;
    let res = clinet
        .customers
        .list_customers(Some(per_page), Some(page))
        .await
        .expect("unable to list customers");

    // Assert
    assert!(res.status);
    assert!(res.message.contains("Customers retrieved"));
    assert!(res.data.unwrap().len() > 0);
}

#[tokio::test]
async fn can_list_customers_in_integration_with_defaults() {
    // Arrange
    let clinet = get_paystack_client();

    // Act
    let res = clinet
        .customers
        .list_customers(None, None)
        .await
        .expect("unable to list customers");

    // Assert
    assert!(res.status);
    assert!(res.message.contains("Customers retrieved"));
    assert!(res.data.unwrap().len() > 0);
}

#[tokio::test]
async fn can_fetch_a_customer_from_the_integration_with_email() {
    // Arrange
    let client = get_paystack_client();

    // create customer
    let body = CreateCustomerRequestBuilder::default()
        .email("test@email.com".to_string())
        .build()
        .unwrap();
    let customer = client
        .customers
        .create_customer(body)
        .await
        .expect("unable to create customer");

    // Act
    let customer_data = customer.data.unwrap();
    let res = client
        .customers
        .fetch_customer(customer_data.email.clone())
        .await
        .expect("unable to fetch customer");

    // Assert
    let res_data = res.data.unwrap();
    assert!(res.status);
    assert_eq!(&res_data.email, &customer_data.email);
    assert_eq!(&res_data.customer_code, &customer_data.customer_code);
}

#[tokio::test]
async fn can_fetch_customer_from_the_integration_with_customer_code() {
    // Arrange
    let client = get_paystack_client();

    // create customer
    let body = CreateCustomerRequestBuilder::default()
        .email("test@email.com".to_string())
        .build()
        .unwrap();
    let customer = client
        .customers
        .create_customer(body)
        .await
        .expect("unable to create customer");

    // Act
    let customer_data = customer.data.unwrap();
    let res = client
        .customers
        .fetch_customer(customer_data.customer_code.clone())
        .await
        .expect("unable to fetch customer");

    // Assert
    let res_data = res.data.unwrap();
    assert!(res.status);
    assert_eq!(&res_data.email, &customer_data.email);
    assert_eq!(&res_data.customer_code, &customer_data.customer_code);
}
