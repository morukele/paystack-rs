pub mod apple_pay;
pub mod customers;
pub mod dedicated_virtual_accounts;
pub mod plans;
pub mod subaccounts;
pub mod subscriptions;
pub mod terminal;
pub mod transactions;
pub mod transaction_splits;
pub mod virtual_terminal;

// public re-export
pub use apple_pay::*;
pub use customers::*;
pub use dedicated_virtual_accounts::*;
pub use plans::*;
pub use subaccounts::*;
pub use subscriptions::*;
pub use terminal::*;
pub use transactions::*;
pub use transaction_splits::*;
pub use virtual_terminal::*;

// Const for the base url, since it is used multiple times
pub const PAYSTACK_BASE_URL: &str = "https://api.paystack.co";
