use fake::Fake;
use fake::faker::company::en::CompanyName;
use fake::faker::lorem::en::{Sentence};
use paystack::{CreateSubaccountBodyBuilder};
use crate::helpers::{get_bank_account_number_and_code, get_paystack_client};

#[tokio::test]
async fn create_subaccount_passes_with_valid_data() {
    // Arrange
    let client = get_paystack_client();

    // Act
    // To test this, we need a life bank account, use the .env file for this
    let business_name: String = CompanyName().fake();
    let description: String = Sentence(5..10).fake();
    let (account_number, bank_code) = get_bank_account_number_and_code();

    let body = CreateSubaccountBodyBuilder::default()
        .business_name(business_name)
        .settlement_bank(bank_code.clone())
        .account_number(account_number.clone())
        .percentage_charge(18.2)
        .description(description)
        .build()
        .unwrap();

    // println!("{:#?}", body);
    let res = client.
        create_subaccount(body)
        .await
        .expect("Unable to Create a subaccount");

    // Assert
    assert!(res.status);
    assert_eq!(res.data.settlement_bank, "Kuda Bank");
    assert_eq!(res.data.account_number, account_number)
}