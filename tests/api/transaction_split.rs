use fake::{
    faker::{company::en::CompanyName, lorem::en::Sentence, name::en::FirstName},
    Fake,
};
use paystack::{
    CreateSubaccountBodyBuilder, CreateTransactionSplitBody, CreateTransactionSplitBodyBuilder,
    Currency, DeleteSubAccountBody, PaystackClient, SubaccountBody, SubaccountBodyBuilder,
    UpdateTransactionSplitBodyBuilder,
};

use crate::helpers::{get_bank_account_number_and_code, get_paystack_client};

async fn create_subaccount_body(
    client: &PaystackClient<'_>,
    percentage_charge: f32,
    share: f32,
) -> SubaccountBody {
    let (account_number, bank_code, _bank_name) = get_bank_account_number_and_code();

    let business_name: String = CompanyName().fake();
    let description: String = Sentence(5..10).fake();

    let body = CreateSubaccountBodyBuilder::default()
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
    client: &PaystackClient<'_>,
) -> (String, CreateTransactionSplitBody) {
    let txn_split_name: String = FirstName().fake();

    // Create first subaccount body
    let first_subaccount_body = create_subaccount_body(client, 18.2, 80.0).await;

    // Create second subaccount body
    let second_subaccount_body = create_subaccount_body(client, 10.0, 10.0).await;

    // Create transaction split body
    let body = CreateTransactionSplitBodyBuilder::default()
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
    let body = CreateTransactionSplitBodyBuilder::default()
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
        assert_eq!(err.to_string(), "Request failed - Status Code: 400 Bad Request Body: {\"status\":false,\"message\":\"At least one subaccount is required\"}".to_string());
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
    let (_, split_body) = build_transaction_split(&client).await;

    // Act
    let transaction_split = client
        .transaction_split
        .create_transaction_split(split_body)
        .await
        .expect("Failed to create transaction split");

    let new_subaccount_body = create_subaccount_body(&client, 44.3, 30.0).await;
    let new_split_name: String = FirstName().fake();

    // create update split body
    let update_split_body = UpdateTransactionSplitBodyBuilder::default()
        .active(false)
        .bearer_type(Some(paystack::BearerType::Account))
        .bearer_subaccount(Some(new_subaccount_body))
        .name(new_split_name.clone())
        .build()
        .unwrap();

    // Act
    let split_id = transaction_split.data.id.to_string();
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

#[tokio::test]
async fn update_a_transaction_split_fails_with_invalid_data() {
    //Arrange
    let client = get_paystack_client();
    let (_, split_body) = build_transaction_split(&client).await;

    // Act
    let transaction_split = client
        .transaction_split
        .create_transaction_split(split_body)
        .await
        .expect("Failed to create transaction split");

    // create update split body
    let update_split_body = UpdateTransactionSplitBodyBuilder::default()
        .active(true)
        .bearer_type(Some(paystack::BearerType::Subaccount))
        .bearer_subaccount(None)
        .name("".to_string())
        .build()
        .unwrap();

    // Act
    let split_id = transaction_split.data.id.to_string();
    let res = client
        .transaction_split
        .update_transaction_split(&split_id, update_split_body)
        .await;

    // Assert
    if let Err(err) = res {
        assert!(err.to_string().contains("Bearer subaccount is required"));
    } else {
        panic!();
    }
}

#[tokio::test]
async fn add_a_transaction_split_subaccount_passes_with_valid_data() {
    // Arrange
    let client = get_paystack_client();
    let (_, split_body) = build_transaction_split(&client).await;

    // Act
    let transaction_split = client
        .transaction_split
        .create_transaction_split(split_body)
        .await
        .expect("Failed to create transaction split");

    let new_subaccount_body = create_subaccount_body(&client, 2.8, 4.0).await;

    let split_id = transaction_split.data.id.to_string();
    let res = client
        .transaction_split
        .add_or_update_subaccount_split(&split_id, new_subaccount_body.clone())
        .await
        .unwrap();

    // Assert
    assert!(res.status);
    assert_eq!(res.message, "Subaccount added");
    assert_eq!(res.data.subaccounts.len(), 3);
}

#[tokio::test]
async fn add_a_transaction_split_subaccount_fails_with_invalid_data() {
    // Arrange
    let client = get_paystack_client();
    let (_, split_body) = build_transaction_split(&client).await;

    // Act
    let transaction_split = client
        .transaction_split
        .create_transaction_split(split_body)
        .await
        .expect("Failed to create transaction split");

    let new_subaccount_body = create_subaccount_body(&client, 55.0, 120.0).await;

    let split_id = transaction_split.data.id.to_string();
    let res = client
        .transaction_split
        .add_or_update_subaccount_split(&split_id, new_subaccount_body.clone())
        .await;

    // Assert
    if let Err(err) = res {
        assert!(err.to_string().contains("Shares cannot exceed 100%"));
    } else {
        panic!();
    };
}

#[tokio::test]
async fn remove_a_subaccount_from_a_transaction_split_passes_with_valid_data() {
    // Arrange
    let client = get_paystack_client();
    let (_, split_body) = build_transaction_split(&client).await;

    // Act
    let transaction_split = client
        .transaction_split
        .create_transaction_split(split_body)
        .await
        .expect("Failed to create transaction split");
    let split_id = transaction_split.data.id.to_string();

    // Validate the number of subaccounts attached
    assert_eq!(transaction_split.data.subaccounts.len(), 2);

    let subaccount_data = transaction_split.data.subaccounts.first().unwrap();
    let code = &subaccount_data.subaccount.subaccount_code;
    // Remove subaccount
    let res = client
        .transaction_split
        .remove_subaccount_from_transaction_split(
            &split_id,
            DeleteSubAccountBody {
                subaccount: code.to_string(),
            },
        )
        .await
        .unwrap();

    // Assert
    assert!(res.status);
    assert_eq!(res.message, "Subaccount removed");

    // Revalidate number of subaccounts attached
    let transaction_split = client
        .transaction_split
        .fetch_transaction_split(&split_id)
        .await
        .unwrap();

    // Assert
    assert!(transaction_split.status);
    assert_eq!(transaction_split.data.total_subaccounts, 1);
    let remaining_subaccount = transaction_split.data.subaccounts.first().unwrap();
    assert_ne!(
        remaining_subaccount.subaccount.subaccount_code,
        subaccount_data.subaccount.subaccount_code
    );
}

#[tokio::test]
async fn remove_a_subaccount_from_a_transaction_split_fails_with_invalid_data() {
    // Arrange
    let client = get_paystack_client();
    let (_, split_body) = build_transaction_split(&client).await;

    // Act
    let transaction_split = client
        .transaction_split
        .create_transaction_split(split_body)
        .await
        .expect("Failed to create transaction split");
    let split_id = transaction_split.data.id.to_string();

    // Validate the number of subaccounts attached
    assert_eq!(transaction_split.data.subaccounts.len(), 2);

    // Remove subaccount
    let res = client
        .transaction_split
        .remove_subaccount_from_transaction_split(
            &split_id,
            DeleteSubAccountBody {
                subaccount: "".to_string(),
            },
        )
        .await;

    // Assert
    if let Err(err) = res {
        assert!(err
            .to_string()
            .contains("Please specify subaccount to be removed"))
    } else {
        panic!();
    }
}
