# paystack-rs

![CI](https://github.com/morukele/paystack-rs/actions/workflows/rust.yml/badge.svg)
[![paystack-rs on crates.io](https://img.shields.io/crates/v/paystack-rs.svg)](https://crates.io/crates/paystack-rs)
[![paystack-rs  on docs.rs](https://docs.rs/paystack-rs/badge.svg)](https://docs.rs/paystack-rs)

Convenient rust bindings and types for the Paystakc HTTP API aiming to support the entire API surface. Not the case? Please open an issue. I update the definitions on a weekly basis.

## Documentation

See the [Rust API docs](https://docs.rs/paystack-rs) or the [examples](/examples).

## Installation

`paystack-rs` uses the `reqwest` http client under the hood and the `tokio` runtime for async operations

```toml
[dependencies]
paystack-rs = "0.1"
```

## Contributing

see [CONTRIBUTING.md](/CONTRIBUTING.md) for information on contributing to paystack-rs.

## License

Licensed under MIT license ([LICENSE-MIT](/LICENSE-MIT)).
