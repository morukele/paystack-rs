//! Paystack Enums
//! ===============
//! This file contains enums of options sent to or returned from the Paystack API

use std::fmt;

use serde::{Deserialize, Serialize};

/// Represents different currencies supported by the Paystack API.
///
/// The `Currency` enum defines the possible currency options that can be used with Paystack,
/// including Nigerian Naira (NGN), Ghanaian Cedis (GHS), American Dollar (USD),
/// and South African Rands (ZAR). It also includes an `EMPTY` variant to represent cases
/// where the currency can be empty.
///
/// # Variants
///
/// - `NGN`: Nigerian Naira.
/// - `GHS`: Ghanaian Cedis.
/// - `USD`: American Dollar.
/// - `ZAR`: South African Rands.
/// - `EMPTY`: Used when the currency can be empty.
///
/// # Examples
///
/// ```
/// use paystack::Currency;
///
/// let ngn_currency = Currency::NGN;
/// let ghs_currency = Currency::GHS;
/// let usd_currency = Currency::USD;
/// let zar_currency = Currency::ZAR;
/// let empty_currency = Currency::EMPTY;
///
/// println!("{:?}", ngn_currency); // Prints: NGN
/// ```
///
/// The example demonstrates the usage of the `Currency` enum from the Paystack crate,
/// creating instances of each variant and printing their debug representation.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub enum Currency {
    /// Nigerian Naira
    #[default]
    NGN,
    /// Ghanaian Cedis
    GHS,
    /// American Dollar
    USD,
    /// South African Rands
    ZAR,
    /// Used when currency can be empty.
    EMPTY,
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let currency = match self {
            Currency::NGN => "NGN",
            Currency::GHS => "GHS",
            Currency::USD => "USD",
            Currency::ZAR => "ZAR",
            Currency::EMPTY => "",
        };
        write!(f, "{}", currency)
    }
}

/// Represents the payment channels supported by Paystack.
///
/// The `Channel` enum defines the possible payment channels that can be used with Paystack,
/// including debit card, bank interface, USSD code, QR code, mobile money, bank transfer,
/// and Apple Pay.
///
/// # Variants
///
/// - `Card`: Payment with a debit card.
/// - `Bank`: Payment with a bank interface.
/// - `Ussd`: Payment with a USSD code.
/// - `Qr`: Payment with a QR code.
/// - `MobileMoney`: Payment with mobile money.
/// - `BankTransfer`: Payment with a bank transfer.
/// - `ApplePay`: Payment with Apple Pay.
///
/// # Examples
///
/// ```
/// use paystack::Channel;
///
/// let card_channel = Channel::Card;
/// let bank_channel = Channel::Bank;
/// let ussd_channel = Channel::Ussd;
/// let qr_channel = Channel::Qr;
/// let mobile_money_channel = Channel::MobileMoney;
/// let bank_transfer_channel = Channel::BankTransfer;
/// let apple_pay_channel = Channel::ApplePay;
///
/// println!("{:?}", card_channel); // Prints: card
/// println!("{:?}", mobile_money_channel); // Prints: mobile_money
/// ```
///
/// The example demonstrates the usage of the `Channel` enum from the Paystack crate,
/// creating instances of each variant and printing their debug representation.
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(rename_all = "snake_case")]
pub enum Channel {
    /// Debit Card
    #[default]
    Card,
    /// Payment with Bank Interface
    Bank,
    /// Payment with USSD Code
    Ussd,
    /// Payment with QR Code
    Qr,
    /// Payment with Mobile Money
    MobileMoney,
    /// Payment with Bank Transfer
    BankTransfer,
    /// Payment with Apple Pay
    ApplePay,
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let lower_case = match self {
            Channel::Card => "card",
            Channel::Bank => "bank",
            Channel::Ussd => "ussd",
            Channel::Qr => "qr",
            Channel::MobileMoney => "mobile_money",
            Channel::BankTransfer => "bank_transfer",
            Channel::ApplePay => "mobile_money",
        };
        write!(f, "{}", lower_case)
    }
}

/// Represents the status of a transaction.
///
/// The `Status` enum defines the possible status values for a transaction,
/// indicating whether the transaction was successful, abandoned, or failed.
///
/// # Variants
///
/// - `Success`: Represents a successful transaction.
/// - `Abandoned`: Represents an abandoned transaction.
/// - `Failed`: Represents a failed transaction.
///
/// # Examples
///
/// ```
/// use paystack::Status;
///
/// let success_status = Status::Success;
/// let abandoned_status = Status::Abandoned;
/// let failed_status = Status::Failed;
///
/// println!("{:?}", success_status); // Prints: Success
/// ```
///
/// The example demonstrates the usage of the `Status` enum, creating instances of each variant
/// and printing their debug representation.
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    /// A successful transaction.
    Success,
    /// An abandoned transaction.
    Abandoned,
    /// A failed transaction.
    Failed,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lowercase_string = match self {
            Status::Success => "success",
            Status::Abandoned => "abandoned",
            Status::Failed => "failed",
        };
        write!(f, "{}", lowercase_string)
    }
}

/// Represents the type of transaction split.
///
/// The `SplitType` enum defines the possible types of transaction splits that can be created,
/// indicating whether the split is based on a percentage or a flat amount.
///
/// # Variants
///
/// - `Percentage`: A split based on a percentage.
/// - `Flat`: A split based on an amount.
///
/// # Examples
///
/// ```
/// use paystack::SplitType;
///
/// let percentage_split = SplitType::Percentage;
/// let flat_split = SplitType::Flat;
///
/// println!("{:?}", percentage_split); // Prints: Percentage
/// ```
///
/// The example demonstrates the usage of the `SplitType` enum, creating instances of each variant
/// and printing their debug representation.
#[derive(Debug, Serialize, Clone, Default)]
#[serde(rename_all = "lowercase")]
pub enum SplitType {
    /// A split based on a percentage
    #[default]
    Percentage,
    /// A split based on an amount
    Flat,
}

impl fmt::Display for SplitType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lowercase_string = match self {
            SplitType::Percentage => "percentage",
            SplitType::Flat => "flat",
        };
        write!(f, "{}", lowercase_string)
    }
}

/// Represents the type of bearer for a charge.
///
/// The `BearerType` enum defines the possible types of bearers for a charge, indicating who
/// is responsible for the transaction split.
///
/// # Variants
///
/// - `Subaccount`: The subaccount bears the transaction split.
/// - `Account`: The main account bears the transaction split.
/// - `AllProportional`: The transaction is split proportionally to all accounts.
/// - `All`: The transaction is paid by all accounts.
///
/// # Examples
///
/// ```
/// use paystack::BearerType;
///
/// let subaccount_bearer = BearerType::Subaccount;
/// let account_bearer = BearerType::Account;
/// let all_proportional_bearer = BearerType::AllProportional;
/// let all_bearer = BearerType::All;
///
/// println!("{:?}", subaccount_bearer); // Prints: Subaccount
/// ```
///
/// The example demonstrates the usage of the `BearerType` enum, creating instances of each variant
/// and printing their debug representation.
#[derive(Debug, Serialize, Clone, Default)]
#[serde(rename_all = "lowercase")]
pub enum BearerType {
    /// The subaccount bears the transaction split
    #[default]
    Subaccount,
    /// The main account bears the transaction split
    Account,
    /// The transaction is split proportionally to all accounts
    #[serde(rename = "all-proportional")]
    AllProportional,
    /// The transaction is paid by all accounts
    All,
}

impl fmt::Display for BearerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lowercase_string = match self {
            BearerType::Subaccount => "subaccount",
            BearerType::Account => "account",
            BearerType::AllProportional => "all-proportional",
            BearerType::All => "all",
        };
        write!(f, "{}", lowercase_string)
    }
}
