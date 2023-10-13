use fake::{faker::name::en::FirstName, Fake};
use paystack::CreateTransactionSplitBodyBuilder;

use crate::helpers::get_bank_account_number_and_code;

#[tokio::test]
async fn create_transaction_split_passes_with_valid_data() {
    let txn_split_name = FirstName().fake();
    let (account_number, bank_code) = get_bank_account_number_and_code();
    let subaccount = paystack::SubaccountBody {
        subaccount: bank_code.clone(),
        share: 20,
    };

    let txn = CreateTransactionSplitBodyBuilder::default()
        .name(txn_split_name)
        .split_type(paystack::SplitType::Flat)
        .currency(paystack::Currency::NGN)
        .bearer_type(paystack::BearerType::All)
        .bearer_subaccount(bank_code)
        .subaccounts(vec![subaccount]);
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
