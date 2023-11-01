use fake::{
    faker::{company::en::CompanyName, lorem::en::Sentence, name::en::FirstName},
    Fake,
};
use paystack::{
    CreateSubaccountBodyBuilder, CreateTransactionSplitBodyBuilder, Currency, SubaccountBodyBuilder,
};

use crate::helpers::{get_bank_account_number_and_code, get_paystack_client};

#[tokio::test]
async fn create_transaction_split_passes_with_valid_data() {
    // Arrange
    let client = get_paystack_client();

    let txn_split_name: String = FirstName().fake();
    let (account_number, bank_code, _bank_name) = get_bank_account_number_and_code();

    // Create first subaccount
    let first_business_name: String = CompanyName().fake();
    let first_description: String = Sentence(5..10).fake();

    let body = CreateSubaccountBodyBuilder::default()
        .business_name(first_business_name)
        .settlement_bank(bank_code.clone())
        .account_number(account_number.clone())
        .percentage_charge(18.2)
        .description(first_description)
        .build()
        .unwrap();

    let first_subaccount = client
        .subaccount
        .create_subaccount(body)
        .await
        .expect("Unable to Create a subaccount");

    // Create second subaccount
    let second_business_name: String = CompanyName().fake();
    let second_description: String = Sentence(5..10).fake();

    let body = CreateSubaccountBodyBuilder::default()
        .business_name(second_business_name)
        .settlement_bank(bank_code.clone())
        .account_number(account_number.clone())
        .percentage_charge(10.0)
        .description(second_description)
        .build()
        .unwrap();

    let second_subaccount = client
        .subaccount
        .create_subaccount(body)
        .await
        .expect("Unable to create a subaccount");

    // Create subaccount bodies
    let first_subaccount_body = SubaccountBodyBuilder::default()
        .share(90.0)
        .subaccount(first_subaccount.data.subaccount_code)
        // .subaccount_code("ACCT_xv2cusld7thdw7r".to_string())
        .build()
        .unwrap();

    let second_subaccount_body = SubaccountBodyBuilder::default()
        .share(10.0)
        .subaccount(second_subaccount.data.subaccount_code)
        .build()
        .unwrap();

    // Create transaction split body
    let split_body = CreateTransactionSplitBodyBuilder::default()
        .name(txn_split_name)
        .split_type(paystack::SplitType::Percentage)
        .currency(paystack::Currency::NGN)
        .bearer_type(paystack::BearerType::Subaccount)
        // .bearer_subaccount("ACCT_xv2cusld7thdw7r")
        .subaccounts(vec![
            first_subaccount_body.clone(),
            second_subaccount_body.clone(),
        ])
        .bearer_subaccount(first_subaccount_body.subaccount)
        .build()
        .unwrap();

    println!("{:#?}", split_body);

    // Act
    let res = client
        .transaction_split
        .create_transaction_split(split_body)
        .await
        .expect("Failed to create transaction split");

    // dbg!(res);

    // Assert
    assert!(res.status);
    assert_eq!(res.message, "Split created");
    assert_eq!(res.data.currency, Currency::NGN.to_string());
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
