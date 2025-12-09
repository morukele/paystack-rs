use fake::{
    faker::{
        internet::en::SafeEmail,
        lorem::fr_fr::Sentence,
        name::en::{FirstName, LastName, Name},
        phone_number::fr_fr::PhoneNumber,
    },
    Fake,
};
use time::{macros::offset, OffsetDateTime, UtcOffset};

use crate::helpers::{generate_random_value, get_paystack_client};
use paystack::{
    CreateCustomerRequestBuilder, Interval, PlanRequestBuilder, SubscriptionRequestBuilder,
};

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

    // we need to create an authorization charge for the customer first

    // 2. create plan
    let plan_name: String = Name().fake();
    let amount: String = generate_random_value(100, 100_000).to_string();
    let interval = Interval::Monthly;
    let description: String = Sentence(4..10).fake();
    let plan_body = PlanRequestBuilder::default()
        .name(plan_name)
        .interval(interval)
        .amount(amount)
        .description(description)
        .build()
        .unwrap();

    let plan = client
        .plans
        .create_plan(plan_body)
        .await
        .expect("unable to create plan");

    // 3. create subscription
    let customer_data = customer.data.unwrap().clone();
    let customer_code = &customer_data.customer_code.clone();
    let plan_code = &plan.data.unwrap().plan_code.clone();
    let authorisation = &customer_data.authorizations.unwrap();
    let mut customer_authorisation = String::new();
    if !authorisation.is_empty() {
        let first = authorisation.first().unwrap();
        if let Some(code) = &first.authorization_code {
            customer_authorisation = code.to_string();
        }
    }
    let subscription_body = SubscriptionRequestBuilder::default()
        .customer(customer_code.clone())
        .plan(plan_code.clone())
        .authorization(customer_authorisation.clone())
        .start_data(OffsetDateTime::now_utc().to_offset(offset!(+1)))
        .build()
        .unwrap();

    let subscription_response = client
        .subscriptions
        .create_subscription(subscription_body)
        .await;

    dbg!("{:?}", &subscription_response);

    // Assert
    // 1. check that status is true
    // assert!(subscription_response.status);
    // 2. check response message
    // 3. check aspect of response data
}
