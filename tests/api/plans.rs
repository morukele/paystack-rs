use fake::{
    faker::{lorem::en::Sentence, name::en::Name},
    Fake,
};
use paystack::{Interval, PlanRequestBuilder, PlanUpdateRequestBuilder};
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
    let interval = Interval::Monthly;
    let description: String = Sentence(4..10).fake();
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
    let interval = Interval::Monthly;
    let description: String = Sentence(4..10).fake();
    let body = PlanRequestBuilder::default()
        .name(name.clone())
        .interval(interval.clone())
        .amount(amount)
        .description(description)
        // TODO: change this if your integration supports this currency
        .currency(paystack::Currency::ZAR)
        .build()
        .unwrap();

    let res = client.plans.create_plan(body).await;

    // Assert
    if let Err(e) = res {
        let res = e.to_string();
        assert!(res.contains("status code: 400 Bad Request"));
    }
}

#[tokio::test]
async fn can_list_all_plans_in_the_integration_with_defaults() {
    // Arrange
    let client = get_paystack_client();

    // Act
    let res = client
        .plans
        .list_plans(None, None, None, None, None)
        .await
        .expect("unable to list plans in the integration");

    // Assert
    assert!(res.status);
    assert_eq!(res.message, "Plans retrieved");
}

#[tokio::test]
async fn can_fetch_plan_with_id() {
    // Arrange
    let client = get_paystack_client();
    let mut rng = rand::thread_rng();
    // create plan
    let name: String = Name().fake();
    let amount: String = rng.gen_range(100..=100_000).to_string();
    let interval = Interval::Monthly;
    let description: String = Sentence(4..10).fake();
    let body = PlanRequestBuilder::default()
        .name(name.clone())
        .interval(interval.clone())
        .amount(amount)
        .description(description)
        .build()
        .unwrap();

    let plan = client
        .plans
        .create_plan(body)
        .await
        .expect("unable to create plan");

    // Act
    let plan_id = plan.data.unwrap().id.to_string();
    let res = client
        .plans
        .fetch_plan(plan_id.clone())
        .await
        .expect("unable to fetch plan with {id}");

    // Assert
    assert!(res.status);
    assert_eq!(res.message, "Plan retrieved");
    assert_eq!(res.data.unwrap().id.to_string(), plan_id);
}

#[tokio::test]
async fn can_fetch_plan_with_code() {
    // Arrange
    let client = get_paystack_client();
    let mut rng = rand::thread_rng();
    // create plan
    let name: String = Name().fake();
    let amount: String = rng.gen_range(100..=100_000).to_string();
    let interval = Interval::Monthly;
    let description: String = Sentence(4..10).fake();
    let body = PlanRequestBuilder::default()
        .name(name.clone())
        .interval(interval.clone())
        .amount(amount)
        .description(description)
        .build()
        .unwrap();

    let plan = client
        .plans
        .create_plan(body)
        .await
        .expect("unable to create plan");

    // Act
    let plan_code = plan.data.unwrap().plan_code.to_string();
    let res = client
        .plans
        .fetch_plan(plan_code.clone())
        .await
        .expect("unable to fetch plan with {code}");

    // Assert
    assert!(res.status);
    assert_eq!(res.message, "Plan retrieved");
    assert_eq!(res.data.unwrap().plan_code.to_string(), plan_code);
}

#[tokio::test]
async fn can_modify_plan_with_plan_code() {
    // Arrange
    let client = get_paystack_client();
    let mut rng = rand::thread_rng();
    // create plan
    let name: String = Name().fake();
    let amount: String = rng.gen_range(100..=100_000).to_string();
    let interval = Interval::Monthly;
    let description: String = Sentence(4..10).fake();
    let body = PlanRequestBuilder::default()
        .name(name.clone())
        .interval(interval.clone())
        .amount(amount)
        .description(description)
        .build()
        .unwrap();

    let plan = client
        .plans
        .create_plan(body)
        .await
        .expect("unable to create plan");

    // Act
    // modify plan
    let new_name: String = Name().fake();
    let new_amount: String = rng.gen_range(100..=100_000).to_string();
    let update_request = PlanUpdateRequestBuilder::default()
        .name(new_name)
        .amount(new_amount)
        .build()
        .unwrap();

    let plan_data = plan.data.unwrap();
    let plan_code = plan_data.plan_code.clone().to_string();
    let res = client
        .plans
        .update_plan(plan_code.clone(), update_request)
        .await
        .expect("unable to update plan with code");

    let updated_plan = client
        .plans
        .fetch_plan(plan_code.clone())
        .await
        .expect("unable to fetch plan with {code}");

    // Assert
    let updated_plan_data = updated_plan.data.unwrap();
    assert!(res.status);
    assert!(res.message.contains("Plan updated."));
    assert_ne!(plan_data.name, updated_plan_data.name);
    assert_ne!(plan_data.amount, updated_plan_data.amount);
}

#[tokio::test]
async fn can_modify_plan_with_plan_id() {
    // Arrange
    let client = get_paystack_client();
    let mut rng = rand::thread_rng();
    // create plan
    let name: String = Name().fake();
    let amount: String = rng.gen_range(100..=100_000).to_string();
    let interval = Interval::Monthly;
    let description: String = Sentence(4..10).fake();
    let body = PlanRequestBuilder::default()
        .name(name.clone())
        .interval(interval.clone())
        .amount(amount)
        .description(description)
        .build()
        .unwrap();

    let plan = client
        .plans
        .create_plan(body)
        .await
        .expect("unable to create plan");

    // Act
    // modify plan
    let new_name: String = Name().fake();
    let new_amount: String = rng.gen_range(100..=100_000).to_string();
    let update_request = PlanUpdateRequestBuilder::default()
        .name(new_name)
        .amount(new_amount)
        .build()
        .unwrap();

    let plan_data = plan.data.unwrap();
    let plan_id = plan_data.id.clone().to_string();
    let res = client
        .plans
        .update_plan(plan_id.clone(), update_request)
        .await
        .expect("unable to update plan with code");

    let updated_plan = client
        .plans
        .fetch_plan(plan_id.clone())
        .await
        .expect("unable to fetch plan with {code}");

    // Assert
    let updated_plan_data = updated_plan.data.unwrap();
    assert!(res.status);
    assert!(res.message.contains("Plan updated."));
    assert_ne!(plan_data.name, updated_plan_data.name);
    assert_ne!(plan_data.amount, updated_plan_data.amount);
}
