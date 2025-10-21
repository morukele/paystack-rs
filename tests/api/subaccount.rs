use fake::{
    faker::{company::zh_tw::CompanyName, lorem::en::Sentence},
    Fake,
};
use paystack::CreateSubaccountRequestBuilder;

use crate::helpers::{get_bank_account_number_and_code, get_paystack_client};

#[tokio::test]
async fn create_a_subaccount() {
    // Arrange
    let client = get_paystack_client();
    let (account_number, bank_code, _) = get_bank_account_number_and_code();

    // Act
    let business_name: String = CompanyName().fake();
    let description: String = Sentence(5..10).fake();
    let body = CreateSubaccountRequestBuilder::default()
        .business_name(business_name.clone())
        .settlement_bank(bank_code.clone())
        .account_number(account_number)
        .percentage_charge(30.0)
        .description(description.clone())
        .build()
        .expect("unable to build sub account request");

    let res = client
        .subaccount
        .create_subaccount(body)
        .await
        .expect("unable to create subaccount");

    // Assert
    assert!(&res.status);
    assert_eq!("Subaccount created", &res.message);
    let data = res.data.unwrap(); // had to unwrap here
    assert_eq!(data.business_name, business_name);
    assert_eq!(data.description.unwrap(), description);
}

#[tokio::test]
async fn list_all_subaccounts_in_the_integration() {
    // Arrange
    let client = get_paystack_client();

    // Act
    let res = client
        .subaccount
        .list_subaccounts(Some(5), None)
        .await
        .expect("unable to get list of subaccounts in the integration");

    // Assert
    assert!(res.status);
    assert_eq!(res.message, "Subaccounts retrieved");
    assert!(!res.data.unwrap().is_empty());
}

#[tokio::test]
async fn fetch_subaccount() {
    // Arrange
    let client = get_paystack_client();

    // get an exisiting subaccount or error out
    let sub_account = client
        .subaccount
        .list_subaccounts(Some(1), None)
        .await
        .expect("unable to get exisiting subaccounts");
    let sub_account_data = sub_account.data.unwrap();
    assert!(
        !sub_account_data.is_empty(),
        "No exisiting subaccounts, create one and try again"
    );

    // get subaccount code from exisiting subaccount
    let sub_account_code = sub_account_data[0].subaccount_code.clone();

    // Act
    let res = client
        .subaccount
        .fetch_subaccount(sub_account_code)
        .await
        .expect("unable to fetch sub account with code");

    // Assert
    assert!(res.status);
    assert_eq!(res.message, "Subaccount retrieved");
    assert_eq!(
        res.data.unwrap().subaccount_code,
        sub_account_data[0].subaccount_code,
    )
}

#[tokio::test]
async fn update_subaccount() {
    // Arrange
    let client = get_paystack_client();

    // get an exisiting subaccount or error out
    let sub_accounts = client
        .subaccount
        .list_subaccounts(Some(2), None)
        .await
        .expect("unable to get exisiting subaccounts");
    let sub_accounts_data = sub_accounts.data.as_ref().unwrap();
    assert!(
        sub_accounts_data.len() > 1,
        "No exisiting subaccounts, create one and try again"
    );

    // get subaccount code and prepare update request
    let sub_account = &sub_accounts_data[0];

    let sub_account_code = sub_account.subaccount_code.clone();
    let update_request = CreateSubaccountRequestBuilder::default()
        .business_name("New business name".to_string())
        .description("new description".to_string())
        .build()
        .expect("unable to build subaccount update request");

    // Act
    let res = client
        .subaccount
        .update_subaccount(sub_account_code, update_request)
        .await
        .expect("unable to update subaccount");

    // Assert
    assert!(res.status);
    assert_eq!(res.message, "Subaccount updated");
    let updated_data = res.data.as_ref().unwrap();
    assert_ne!(updated_data.business_name, sub_account.business_name);
    assert_eq!(updated_data.business_name, "New business name");
}
