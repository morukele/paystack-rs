//! Transaction
//! ===========
//!
//! Reference: <https://paystack.com/docs/api/transaction/>
//!
//! This example shows how to initiate a transaction
//! for a particular price and a particular customer.
//! The transaction generates a URL that the user can use to pay.
//! This reqires building a transaction body.
//! Please see the type definition to understand how it is constructed

use dotenv::dotenv;
use paystack::{Channel, Currency, PaystackClient, Status, TransactionBuilder};
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("PAYSTACK_API_KEY").unwrap();
    let client = PaystackClient::new(api_key);

    let body = TransactionBuilder::new()
        .email("email@example.com")
        .amount("200000")
        .currency(Currency::NGN)
        .channels(vec![Channel::Qr, Channel::Ussd, Channel::BankTransfer])
        .build()
        .unwrap();

    let transaction = client
        .initialize_transaction(body)
        .await
        .expect("Unable to create transaction");

    println!(
        "Created a payment payment URL: {}",
        transaction.data.authorization_url
    );
    println!("Transaction creation status: {}", transaction.status);
    println!("Transaction message: {}", transaction.message);

    // Verify transaction
    // Transaction reference can be a string or pulled out from the transaction response
    let transaction_status = client
        .verify_transaction(transaction.data.reference.to_string())
        .await
        .expect("Unable to get transaction status");

    println!("Status: {}", transaction_status.data.status.unwrap());
    println!(
        "Amount of {} {}",
        transaction_status.data.amount.unwrap(),
        transaction_status.data.currency.unwrap()
    );
    println!("Channel: {}", transaction_status.data.channel.unwrap());

    // List of transactiosn
    let transactions = client
        .list_transactions(Some(5), Some(Status::Success))
        .await
        .expect("Unable to get all the transactions");

    println!(
        "{} transactions retrieved from the integration.",
        transactions.data.len()
    );
}
