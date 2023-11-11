# paystack-rs

[![Rust](https://github.com/morukele/paystack-rs/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/morukele/paystack-rs/actions/workflows/rust.yml)
[![paystack-rs on crates.io](https://img.shields.io/crates/v/paystack-rs.svg)](https://crates.io/crates/paystack-rs)
[![paystack-rs  on docs.rs](https://docs.rs/paystack-rs/badge.svg)](https://docs.rs/paystack-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Convenient **Async** rust bindings and types for the [Paystack](https://paystack.com) HTTP API aiming to support the entire API surface. Not the case? Please open an issue. I update the definitions on a weekly basis.

The client aims to make receiving payments for African business or business with African clients building with Rust as hassle-free as possible.

The client currently covers the following section of the API, and the sections to be implemented in order are left unchecked:

- [x] Transaction
- [x] Transaction Split
- [x] Terminal
- [ ] Customers
- [ ] Dedicated Virtual Account
- [ ] Apple Pay
- [x] Subaccounts
- [ ] Plans
- [ ] Subscriptions
- [ ] Transfer Recipients
- [ ] Transfers
- [ ] Transfers Control
- [ ] Bulk Charges
- [ ] Integration
- [ ] Charge
- [ ] Disputes
- [ ] Refunds
- [ ] Verifications
- [ ] Miscellaneous

## Documentation

See the [Rust API docs](https://docs.rs/paystack-rs) or the [examples](/examples).

## Installation

`paystack-rs` uses the `reqwest` HTTP client under the hood and the `tokio` runtime for async operations.

```toml
[dependencies]
paystack-rs = "0.X.X"
```

You can also download the source code and use in your code base directly if you prefer.

## Usage

Initializing an instance of the Paystack client and creating a transaction.

```rust
use std::env; 
use dotenv::dotenv; 
use paystack::{PaystackClient, InitializeTransactionBodyBuilder, Error, Currency, Channel};

#[tokio::main]
async fn main() -> Result<(), Error>{
    dotenv().ok();
    let api_key = env::var("PAYSTACK_API_KEY").unwrap();
    let client = PaystackClient::new(api_key);

    let body = InitializeTransactionBodyBuilder::default()
                .amount("10000".to_string())
                .email("email@example.com".to_string())
                .currency(Some(Currency::NGN))
                .channels(Some(vec![
                    Channel::ApplePay,
                    Channel::Bank,
                    Channel::BankTransfer
                ]))
                .build()
                .unwrap();

    let transaction = client
        .transaction
        .initialize_transaction(body)
        .await
        .expect("Unable to create transaction");
         
    Ok(())
}
```

## Contributing

See [CONTRIBUTING.md](/CONTRIBUTING.md) for information on contributing to paystack-rs.

## License

Licensed under MIT license ([LICENSE-MIT](/LICENSE-MIT)).
