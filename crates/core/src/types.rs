//! Defines all **canonical data structures** used for communication between the
//! `merchant-rs-core` and its gateway adapters.
//!
//! This module ensures **type safety** and consistency across all financial operations.
//! It includes fundamental types for transactions (requests/responses), financial
//! entities (currencies, amounts, tokens), and payment sources (cards, bank accounts).
//!
//! By making these structures canonical, the core decouples the business logic
//! from the specific data formats required by external Payment Gateways (PAGs),
//! upholding the core's role as a stable abstraction layer.

mod account_number;
mod authorization_code;
mod authorization_id;
mod bank_name;
mod card_expiry;
mod card_holder_name;
mod customer_id;
mod cvv;
mod merchant_reference_id;
mod payment_token;
mod primary_account_number;
mod reason_for_refund;
mod refund_id;
mod routing_number;
mod transaction_id;

use strum_macros::{AsRefStr, Display};

pub use account_number::AccountNumber;
pub use authorization_code::AuthorizationCode;
pub use authorization_id::AuthorizationId;
pub use card_expiry::CardExpiry;
pub use card_holder_name::CardHolderName;
pub use customer_id::CustomerId;
pub use cvv::CVV;
pub use iso_currency::Currency;
pub use merchant_reference_id::MerchantReferenceId;
pub use payment_token::PaymentToken;
pub use primary_account_number::PrimaryAccountNumber;
pub use reason_for_refund::ReasonForRefund;
pub use refund_id::RefundId;
pub use routing_number::RoutingNumber;
pub use rust_decimal::Decimal;
pub use transaction_id::TransactionId;

/// A string representing the name of a bank.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BankName(String);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Money {
    pub amount: Decimal,
    pub currency: Currency,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Display, AsRefStr)]
pub enum TransactionStatus {
    Authorized,
    Captured,
    Pending,
    Declined,
    Failed,
    Voided,
    Refunded,
    Processing,
}

#[derive(Debug, Clone)]
pub enum PaymentSource {
    Token(PaymentToken),
    BankAccount(BankAccountDetails),
    Other(String),
}

#[derive(Debug, Clone)]
pub struct BankAccountDetails {
    pub account_number: AccountNumber,
    pub routing_number: RoutingNumber,
    pub bank_name: Option<BankName>,
}

/// Details about a credit or debit card (Used ONLY for initial tokenization requests).
#[derive(Debug)]
pub struct CardDetails {
    /// The primary account number (PAN).
    pub number: PrimaryAccountNumber,
    /// The card's expiration time (month and year).
    pub card_expiry: CardExpiry,
    /// The card verification value (CVV/CVC), optional for vaulted cards.
    pub cvv: Option<CVV>,
    /// The name of the cardholder, optional for most APIs.
    pub holder_name: Option<CardHolderName>,
}
