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
mod card_expiry;
mod cvv;
mod payment_token;
mod primary_account_number;
mod routing_number;
mod secret_string;

use crate::error::{Error, ErrorCode, Result};
use secret_string::SecretString;
use std::fmt;
use strum_macros::{AsRefStr, Display};

pub use iso_currency::Currency;
pub use rust_decimal::Decimal;

pub use account_number::AccountNumber;
pub use card_expiry::CardExpiry;
pub use cvv::CVV;
pub use payment_token::PaymentToken;
pub use primary_account_number::PrimaryAccountNumber;
pub use routing_number::RoutingNumber;

/// A unique identifier for a customer within the payment gateway's vault.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct CustomerId(String);

/// The name of the cardholder.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CardHolderName(String);

/// A unique identifier assigned by the merchant for a transaction.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct MerchantReferenceId(String);

/// A unique identifier assigned by the payment gateway for a transaction.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct TransactionId(String);

/// A unique identifier of the refund.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct RefundId(String);

/// A unique identifier for an authorization transaction.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct AuthorizationId(String);

/// A reason for initiating a refund.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReasonForRefund(String);

/// A code returned by the bank upon successful authorization.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct AuthorizationCode(String);

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
