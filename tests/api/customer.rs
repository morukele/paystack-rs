use fake::{
    faker::{
        internet::en::SafeEmail,
        name::en::{FirstName, LastName},
        phone_number::fr_fr::PhoneNumber,
    },
    Fake,
};
use paystack::{
    CreateCustomerRequestBuilder, IdentificationType, UpdateCustomerRequestBuilder,
    ValidateCustomerRequestBuilder,
};

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

// TODO: find a way to clean up customers in the integration after the test
#[tokio::test]
async fn can_modify_customer_information() {
    // Arrange
    let client = get_paystack_client();

    // Act
    let body = CreateCustomerRequestBuilder::default()
        .email("test@email.com".to_string())
        .first_name("Old First Name".to_string())
        .last_name("Old Last Name".to_string())
        .build()
        .unwrap();
    let customer = client
        .customers
        .create_customer(body)
        .await
        .expect("unable to create customer");

    let customer_data = customer.data.unwrap();

    // Check that these fields don't exist
    assert!(customer.status);
    assert!(customer.message.contains("Customer created"));

    // update customer
    let update_request = UpdateCustomerRequestBuilder::default()
        .first_name("New First Name".to_string())
        .last_name("New Last Name".to_string())
        .build()
        .unwrap();
    let updated_customer = client
        .customers
        .update_customer(customer_data.customer_code, update_request)
        .await
        .expect("unable to update customer");

    // Assert
    let updated_customer_data = updated_customer.data.unwrap();
    assert!(updated_customer.status);
    assert!(updated_customer.message.contains("Customer updated"));
    assert_eq!(
        updated_customer_data.first_name,
        Some("New First Name".to_string())
    );
    assert_eq!(
        updated_customer_data.last_name,
        Some("New Last Name".to_string())
    )
}

#[tokio::test]
async fn can_initiate_customer_validation_request() {
    // Arrange
    let client = get_paystack_client();
    let first_name: String = FirstName().fake();
    let last_name: String = LastName().fake();
    let middle_name: String = FirstName().fake();
    let email: String = SafeEmail().fake();

    // Act
    // create customer
    let body = CreateCustomerRequestBuilder::default()
        .email(email)
        .first_name(first_name.clone())
        .last_name(last_name.clone())
        .build()
        .unwrap();
    let customer = client
        .customers
        .create_customer(body)
        .await
        .expect("unable to create customer");
    let customer_data = customer.data.unwrap();

    // validate customer
    let customer_validation_request = ValidateCustomerRequestBuilder::default()
        .country("NG".to_string())
        .identification_type(IdentificationType::BankAccount)
        .account_number("0123456789".to_string())
        .bvn("20012345677".to_string())
        .bank_code("007".to_string())
        .first_name(first_name.clone())
        .last_name(last_name.clone())
        .middle_name(middle_name)
        .build()
        .unwrap();

    let validation_response = client
        .customers
        .validate_customer(customer_data.customer_code, customer_validation_request)
        .await
        .expect("Unable to validate customer");

    // Assert
    assert!(validation_response.status);
    assert!(validation_response
        .message
        .contains("Customer Identification in progress"))
}
