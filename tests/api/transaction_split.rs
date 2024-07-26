use crate::helpers::{get_bank_account_number_and_code, get_paystack_client};
use fake::{
    faker::{company::en::CompanyName, lorem::en::Sentence, name::en::FirstName},
    Fake,
};
use paystack::{
    Currency, PaystackClient, ReqwestClient, SubaccountBody, SubaccountBodyBuilder,
    SubaccountRequestBuilder, TransactionSplitRequest, TransactionSplitRequestBuilder,
};

async fn create_subaccount_body(
    client: &PaystackClient<ReqwestClient>,
    percentage_charge: f32,
    share: f32,
) -> SubaccountBody {
    let (account_number, bank_code, _bank_name) = get_bank_account_number_and_code();

    let business_name: String = CompanyName().fake();
    let description: String = Sentence(5..10).fake();

    let body = SubaccountRequestBuilder::default()
        .business_name(business_name)
        .settlement_bank(bank_code.clone())
        .account_number(account_number.clone())
        .percentage_charge(percentage_charge)
        .description(description)
        .build()
        .unwrap();

    let subaccount = client
        .subaccount
        .create_subaccount(body)
        .await
        .expect("Unable to Create a subaccount");

    SubaccountBodyBuilder::default()
        .share(share)
        .subaccount(subaccount.data.subaccount_code)
        .build()
        .unwrap()
}

async fn build_transaction_split(
    client: &PaystackClient<ReqwestClient>,
) -> (String, TransactionSplitRequest) {
    let txn_split_name: String = FirstName().fake();

    // Create first subaccount body
    let first_subaccount_body = create_subaccount_body(client, 18.2, 80.0).await;

    // Create second subaccount body
    let second_subaccount_body = create_subaccount_body(client, 10.0, 10.0).await;

    // Create transaction split body
    let body = TransactionSplitRequestBuilder::default()
        .name(txn_split_name.clone())
        .split_type(paystack::SplitType::Percentage)
        .currency(paystack::Currency::NGN)
        .bearer_type(paystack::BearerType::Subaccount)
        .subaccounts(vec![
            first_subaccount_body.clone(),
            second_subaccount_body.clone(),
        ])
        .bearer_subaccount(first_subaccount_body.subaccount)
        .build()
        .unwrap();

    (txn_split_name, body)
}

#[tokio::test]
async fn create_transaction_split_passes_with_valid_data() {
    // Arrange
    let client = get_paystack_client();
    let (_, split_body) = build_transaction_split(&client).await;

    // Act
    let res = client
        .transaction_split
        .create_transaction_split(split_body)
        .await
        .expect("Failed to create transaction split");

    // Assert
    assert!(res.status);
    assert_eq!(res.message, "Split created");
    assert_eq!(res.data.currency, Currency::NGN.to_string());
}

#[tokio::test]
async fn create_transaction_split_fails_with_invalid_data() {
    //Arrange
    let client = get_paystack_client();
    let split_name: String = FirstName().fake();
    let body = TransactionSplitRequestBuilder::default()
        .name(split_name)
        .split_type(paystack::SplitType::Flat)
        .currency(paystack::Currency::EMPTY)
        .subaccounts(vec![])
        .bearer_type(paystack::BearerType::Subaccount)
        .bearer_subaccount("non_existent_subaccount".to_string())
        .build()
        .unwrap();

    //Act
    let res = client
        .transaction_split
        .create_transaction_split(body)
        .await;

    if let Err(err) = res {
        assert!(err.to_string().contains("status code: 400 Bad Request"));
    } else {
        panic!();
    }
}
