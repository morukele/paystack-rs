use crate::helpers::{get_bank_account_number_and_code, get_paystack_client};
use fake::{
    faker::{company::en::CompanyName, lorem::en::Sentence, name::en::FirstName},
    Fake,
};
use paystack::{
    Currency, PaystackClient, ReqwestClient, SubaccountBody, SubaccountBodyBuilder,
    SubaccountRequestBuilder, TransactionSplitRequest, TransactionSplitRequestBuilder,
    UpdateTransactionSplitRequestBuilder,
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

#[tokio::test]
async fn list_transaction_splits_in_the_integration() {
    // Arrange
    let client = get_paystack_client();
    let (split_name, split_body) = build_transaction_split(&client).await;

    // Act
    // Create transaction split
    client
        .transaction_split
        .create_transaction_split(split_body)
        .await
        .expect("Failed to create transaction split");

    // Fetch the splits
    let res = client
        .transaction_split
        .list_transaction_splits(Some(&split_name), None)
        .await;

    // Assert
    if let Ok(data) = res {
        assert!(data.status);
        assert_eq!(data.message, "Split retrieved".to_string());
        assert_eq!(data.data.len(), 1);

        let transaction_split = data.data.first().unwrap();
        assert_eq!(
            transaction_split.split_type,
            paystack::SplitType::Percentage.to_string()
        );
    } else {
        dbg!("response: {:?}", &res);
        panic!();
    }
}

#[tokio::test]
async fn fetch_a_transaction_split_in_the_integration() {
    //Arrange
    let client = get_paystack_client();
    let (_, split_body) = build_transaction_split(&client).await;

    // Act
    let transaction_split = client
        .transaction_split
        .create_transaction_split(split_body)
        .await
        .expect("Failed to create transaction split");

    let res = client
        .transaction_split
        .fetch_transaction_split(&transaction_split.data.id.to_string())
        .await
        .unwrap();

    // Assert
    dbg!(&res);
    assert!(res.status);
    assert_eq!(
        res.data.total_subaccounts as usize,
        res.data.subaccounts.len()
    );
    assert_eq!(res.message, "Split retrieved".to_string());
}

#[tokio::test]
async fn update_a_transaction_split_passes_with_valid_data() {
    //Arrange
    let client = get_paystack_client();
    let transaction_split = client
        .transaction_split
        .list_transaction_splits(None, Some(true))
        .await
        .expect("Failed to create transaction split");

    // Act
    let new_subaccount_body = create_subaccount_body(&client, 44.3, 30.0).await;
    let new_split_name: String = FirstName().fake();

    // create update split body
    let update_split_body = UpdateTransactionSplitRequestBuilder::default()
        .active(false)
        .bearer_type(paystack::BearerType::Account)
        .bearer_subaccount(new_subaccount_body)
        .name(new_split_name.clone())
        .build()
        .unwrap();

    // Act
    let split_id = transaction_split.data[0].id.to_string();
    let res = client
        .transaction_split
        .update_transaction_split(&split_id, update_split_body)
        .await;

    // Assert
    if let Ok(data) = res {
        assert!(data.status);
        assert_eq!(data.message, "Split group updated".to_string());
        assert!(!data.data.active.unwrap());
        assert_eq!(data.data.name, new_split_name);
    } else {
        panic!();
    }
}
