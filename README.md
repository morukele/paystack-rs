# paystack-rs

![CI](https://github.com/morukele/paystack-rs/actions/workflows/rust.yml/badge.svg)
[![paystack-rs on crates.io](https://img.shields.io/crates/v/paystack-rs.svg)](https://crates.io/crates/paystack-rs)
[![paystack-rs  on docs.rs](https://docs.rs/paystack-rs/badge.svg)](https://docs.rs/paystack-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Convenient **Async** rust bindings and types for the Paystack HTTP API aiming to support the entire API surface. Not the case? Please open an issue. I update the definitions on a weekly basis.

The client current covers the follow section of the API:

- Transactions
- Transaction Splits

## Documentation

See the [Rust API docs](https://docs.rs/paystack-rs) or the [examples](/examples).

## Installation

`paystack-rs` uses the `reqwest` http client under the hood and the `tokio` runtime for async operations.

```toml
[dependencies]
paystack-rs = "0.1"
```

You can also download the source code and use in your code base directly if you prefer.

## Usage

Initalizing an instance of the Paystack client and creating a transaction.

```rust
    use std::env;
    use dotenv::dotenv;
    use paystack::{PaystackClient, TransactionBuilder, PaystackError};

    #[tokio::main]
    async fn main() -> Result<(), PaystackError>{
        dotenv().ok()
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

        Ok(())
    }
```

## Contributing

See [CONTRIBUTING.md](/CONTRIBUTING.md) for information on contributing to paystack-rs.

## License

Licensed under MIT license ([LICENSE-MIT](/LICENSE-MIT)).
