pub mod apple_pay;
pub mod customers;
pub mod dedicated_virtual_account;
pub mod subaccount;
pub mod terminal;
pub mod transaction;
pub mod transaction_split;
pub mod virtual_terminal;

// public re-export
pub use apple_pay::*;
pub use customers::*;
pub use dedicated_virtual_account::*;
pub use subaccount::*;
pub use terminal::*;
pub use transaction::*;
pub use transaction_split::*;
pub use virtual_terminal::*;

// Const for the base url, since it is used multiple times
pub const BASE_URL: &str = "https://api.paystack.co";
