use fake::{
    faker::{lorem::en::Paragraph, name::en::Name},
    Fake,
};
use paystack::{Interval, PlanRequestBuilder};
use rand::Rng;

use crate::helpers::get_paystack_client;

#[tokio::test]
async fn create_plan_valid() {
    // Arrange
    let client = get_paystack_client();
    let mut rng = rand::thread_rng();

    // Act
    let name: String = Name().fake();
    let amount: String = rng.gen_range(100..=100_000).to_string();
    let interval = Interval::MONTHLY;
    let description: String = Paragraph(0..2).fake();
    let body = PlanRequestBuilder::default()
        .name(name.clone())
        .interval(interval.clone())
        .amount(amount)
        .description(description)
        .build()
        .unwrap();

    let res = client
        .plans
        .create_plan(body)
        .await
        .expect("unable to create plan");

    // Assert
    assert!(res.status);
    assert_eq!("Plan created", res.message);
    let data = res.data.unwrap();
    assert_eq!(&data.name, &name);
    assert_eq!(&data.interval, &interval);
}

#[tokio::test]
async fn create_plan_fails_when_currency_is_not_supported_by_merchant() {
    // Arrange
    let client = get_paystack_client();
    let mut rng = rand::thread_rng();

    // Act
    let name: String = Name().fake();
    let amount: String = rng.gen_range(100..=100_000).to_string();
    let interval = Interval::MONTHLY;
    let description: String = Paragraph(0..2).fake();
    let body = PlanRequestBuilder::default()
        .name(name.clone())
        .interval(interval.clone())
        .amount(amount)
        .description(description)
        .currency(paystack::Currency::ZAR) // TODO: change this if your integration supports this currency
        .build()
        .unwrap();

    let res = client.plans.create_plan(body).await;

    // Assert
    if let Err(e) = res {
        let res = e.to_string();
        assert!(res.contains("status code: 400 Bad Request"));
    }
}
