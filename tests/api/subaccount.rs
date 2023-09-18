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
    let res = client
        .create_subaccount(body)
        .await
        .expect("Unable to Create a subaccount");

    // Assert
    assert!(res.status);
    assert_eq!(res.data.settlement_bank, "Kuda Bank");
    assert_eq!(res.data.account_number, account_number)
}

#[tokio::test]
async fn create_subaccount_fails_with_invalid_data() {
    // Arrange
    let client = get_paystack_client();

    // Act
    let body = CreateSubaccountBodyBuilder::default()
        .business_name("".to_string())
        .settlement_bank("".to_string())
        .account_number("".to_string())
        .description("".to_string())
        .percentage_charge(0.0)
        .build()
        .unwrap();

    let res = client.create_subaccount(body).await;

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