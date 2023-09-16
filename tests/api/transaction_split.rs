use fake::Fake;
use fake::faker::name::en::Name;
use paystack::{BearerType, CreateTransactionSplitBodyBuilder, Currency, SplitType};
use crate::helpers::get_paystack_client;

#[tokio::test]
async fn create_transaction_split_passes_with_valid_data() {
    // Arrange
    let client = get_paystack_client();

    // Act
    let name: String = Name().fake();
    let body = CreateTransactionSplitBodyBuilder::default()
        .name(name)
        .split_type(SplitType::Percentage)
        .currency(Currency::NGN)
        .subaccounts(vec![])
        .bearer_type(BearerType::Subaccount)
        .bearer_subaccount("".to_string())
        .build()
        .unwrap();
    println!("{:#?}", body);
    // Assert
}

#[tokio::test]
async fn create_transaction_split_fails_with_invalid_data() {}

#[tokio::test]
async fn list_transaction_splits_in_the_integration() {}

#[tokio::test]
async fn fetch_a_transaction_split_in_the_integration() {}

#[tokio::test]
async fn update_a_transaction_split_passes_with_valid_data() {}

#[tokio::test]
async fn update_a_transaction_split_fails_with_invalid_data() {}

#[tokio::test]
async fn add_a_transaction_split_subaccount_passes_with_valid_data() {}

#[tokio::test]
async fn add_a_transaction_split_subaccount_fails_with_invalid_data() {}

#[tokio::test]
async fn remove_a_subaccount_from_a_transaction_split_passes_with_valid_data() {}

#[tokio::test]
async fn remove_a_subaccount_from_a_transaction_split_fails_with_invalid_data() {}
