use crate::helpers::{get_bank_account_number_and_code, get_paystack_client};
use fake::faker::company::en::CompanyName;
use fake::faker::lorem::en::Sentence;
use fake::Fake;
use paystack::CreateSubaccountBodyBuilder;

#[tokio::test]
async fn create_subaccount_passes_with_valid_data() {
    // Arrange
    let client = get_paystack_client();

    // Act
    // To test this, we need a life bank account, use the .env file for this
    let business_name: String = CompanyName().fake();
    let description: String = Sentence(5..10).fake();
    let (account_number, bank_code, bank_name) = get_bank_account_number_and_code();

    let body = CreateSubaccountBodyBuilder::default()
        .business_name(&business_name)
        .settlement_bank(&bank_code)
        .account_number(&account_number)
        .percentage_charge(18.2)
        .description(&description)
        .build()
        .unwrap();

    // println!("{:#?}", body);
    let res = client
        .subaccount
        .create_subaccount(body)
        .await
        .expect("Unable to Create a subaccount");

    // Assert
    assert!(res.status);
    assert_eq!(res.data.settlement_bank, bank_name);
    assert_eq!(res.data.account_number, account_number)
}

#[tokio::test]
async fn create_subaccount_fails_with_invalid_data() {
    // Arrange
    let client = get_paystack_client();

    // Act
    let body = CreateSubaccountBodyBuilder::default()
        .business_name("")
        .settlement_bank("")
        .account_number("")
        .description("")
        .percentage_charge(0.0)
        .build()
        .unwrap();

    let res = client.subaccount.create_subaccount(body).await;

    // Assert
    match res {
        Ok(_) => (),
        Err(e) => {
            let res = e.to_string();
            // dbg!("{:#?}", &res);
            assert!(res.contains("Status Code: 400 Bad Request"));
            assert!(res.contains("Account number is required"))
        }
    }
}

#[tokio::test]
async fn list_subaccounts_returns_a_valid_payload() {
    // Arrange
    let client = get_paystack_client();

    // Act
    let res = client
        .subaccount
        .list_subaccounts(Some(10), Some(1), None, None)
        .await
        .expect("Unable to list subaccounts");

    // Assert
    assert!(res.status);
    assert!(!res.data.is_empty());
    assert_eq!(res.message, "Subaccounts retrieved");
}

#[tokio::test]
async fn fetch_subaccount_with_id_returns_a_valid_payload() {
    // Arrange
    let client = get_paystack_client();
    let business_name: String = CompanyName().fake();
    let description: String = Sentence(5..10).fake();
    let (account_number, bank_code, bank_name) = get_bank_account_number_and_code();

    // Act
    let body = CreateSubaccountBodyBuilder::default()
        .business_name(&business_name)
        .settlement_bank(&bank_code)
        .account_number(&account_number)
        .percentage_charge(18.2)
        .description(&description)
        .build()
        .unwrap();

    // println!("{:#?}", body);
    let subaccount = client
        .subaccount
        .create_subaccount(body)
        .await
        .expect("Unable to Create a subaccount");

    let res = client
        .subaccount
        .fetch_subaccount(&subaccount.data.id.to_string())
        .await
        .expect("Unable to fetch subaccount");

    // Assert
    assert!(res.status);
    assert_eq!(res.message, "Subaccount retrieved");
    assert_eq!(res.data.account_number, subaccount.data.account_number);
}

#[tokio::test]
async fn fetch_subaccount_with_subaccount_code_returns_a_valid_payload() {
    // Arrange
    let client = get_paystack_client();
    let business_name: String = CompanyName().fake();
    let description: String = Sentence(5..10).fake();
    let (account_number, bank_code, bank_name) = get_bank_account_number_and_code();

    // Act
    let body = CreateSubaccountBodyBuilder::default()
        .business_name(&business_name)
        .settlement_bank(&bank_code)
        .account_number(&account_number)
        .percentage_charge(18.2)
        .description(&description)
        .build()
        .unwrap();

    // println!("{:#?}", body);
    let subaccount = client
        .subaccount
        .create_subaccount(body)
        .await
        .expect("Unable to Create a subaccount");

    let res = client
        .subaccount
        .fetch_subaccount(&subaccount.data.subaccount_code)
        .await
        .expect("Unable to fetch subaccount");

    // Assert
    assert!(res.status);
    assert_eq!(res.message, "Subaccount retrieved");
    assert_eq!(res.data.account_number, subaccount.data.account_number);
}

#[tokio::test]
async fn modify_subaccount_with_subaccount_id_returns_a_valid_payload() {
    // Arrange
    let client = get_paystack_client();
    let business_name: String = CompanyName().fake();
    let description: String = Sentence(5..10).fake();
    let (account_number, bank_code, bank_name) = get_bank_account_number_and_code();

    // Act
    let body = CreateSubaccountBodyBuilder::default()
        .business_name(&business_name)
        .settlement_bank(&bank_code)
        .account_number(&account_number)
        .percentage_charge(18.2)
        .description(&description)
        .build()
        .unwrap();

    // println!("{:#?}", body);
    let subaccount = client
        .subaccount
        .create_subaccount(body)
        .await
        .expect("Unable to Create a subaccount");

    let new_body = CreateSubaccountBodyBuilder::default()
        .business_name("New Business Name")
        .settlement_bank(&bank_code)
        .account_number(&account_number)
        .percentage_charge(18.2)
        .description("This should be modified")
        .build()
        .unwrap();

    let res = client
        .subaccount
        .update_subaccount(&subaccount.data.subaccount_code, new_body)
        .await
        .expect("Unable to fetch subaccount");

    // Assert
    assert!(res.status);
    assert_eq!(res.message, "Subaccount updated");
    assert_eq!(res.data.description.unwrap(), "This should be modified");
    assert_eq!(res.data.business_name, "New Business Name")
}

#[tokio::test]
async fn modify_subaccount_with_subaccount_code_returns_a_valid_payload() {
    // Arrange
    let client = get_paystack_client();
    let business_name: String = CompanyName().fake();
    let description: String = Sentence(5..10).fake();
    let (account_number, bank_code, bank_name) = get_bank_account_number_and_code();

    // Act
    let body = CreateSubaccountBodyBuilder::default()
        .business_name(&business_name)
        .settlement_bank(&bank_code)
        .account_number(&account_number)
        .percentage_charge(18.2)
        .description(&description)
        .build()
        .unwrap();

    // println!("{:#?}", body);
    let subaccount = client
        .subaccount
        .create_subaccount(body)
        .await
        .expect("Unable to Create a subaccount");

    let new_body = CreateSubaccountBodyBuilder::default()
        .business_name("New Business Name")
        .settlement_bank(&bank_code)
        .account_number(&account_number)
        .percentage_charge(18.2)
        .description("This should be modified")
        .build()
        .unwrap();

    let res = client
        .subaccount
        .update_subaccount(&subaccount.data.id.to_string(), new_body)
        .await
        .expect("Unable to fetch subaccount");

    // Assert
    assert!(res.status);
    assert_eq!(res.message, "Subaccount updated");
    assert_eq!(res.data.description.unwrap(), "This should be modified");
    assert_eq!(res.data.business_name, "New Business Name")
}
