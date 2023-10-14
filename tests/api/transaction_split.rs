use std::{thread, time};

use fake::{
    faker::{company::en::CompanyName, lorem::en::Sentence, name::en::FirstName},
    Fake,
};
use paystack::{
    CreateSubaccountBodyBuilder, CreateTransactionSplitBodyBuilder, SubaccountBodyBuilder,
};

use crate::helpers::{get_bank_account_number_and_code, get_paystack_client};

#[tokio::test]
async fn create_transaction_split_passes_with_valid_data() {
    // Arrange
    let client = get_paystack_client();

    let txn_split_name = FirstName().fake();
    let (account_number, bank_code, bank_name) = get_bank_account_number_and_code();

    let business_name: String = CompanyName().fake();
    let description: String = Sentence(5..10).fake();

    let body = CreateSubaccountBodyBuilder::default()
        .business_name(&business_name)
        .settlement_bank(&bank_code)
        .account_number(&account_number)
        .percentage_charge(18.2)
        .description(&description)
        .build()
        .unwrap();

    let subaccount = client
        .subaccount
        .create_subaccount(body)
        .await
        .expect("Unable to Create a subaccount");

    println!("{:#?}", subaccount);

    let subaccount_body = SubaccountBodyBuilder::default()
        .share(90.0)
        .subaccount_code(subaccount.data.subaccount_code.clone())
        // .subaccount_code("ACCT_xv2cusld7thdw7r".to_string())
        .build()
        .unwrap();

    let split_body = CreateTransactionSplitBodyBuilder::default()
        .name(txn_split_name)
        .split_type(paystack::SplitType::Percentage)
        .currency(paystack::Currency::NGN)
        .bearer_type(paystack::BearerType::Subaccount)
        // .bearer_subaccount("ACCT_xv2cusld7thdw7r")
        .subaccounts(vec![subaccount_body.clone()])
        .bearer_subaccount(&subaccount_body.subaccount_code)
        .build()
        .unwrap();

    println!("{:#?}", split_body);

    // Act
    let res = client
        .transaction_split
        .create_transaction_split(split_body)
        .await
        .expect("Failed to create transaction split");
    println!("{:#?}", res);
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
