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

use crate::error::Error;
use serde::{Deserialize, Serialize};
use strum_macros::{AsRefStr, Display};

pub use iso_currency::Currency;
pub use rust_decimal::Decimal;

/// Canonical representation of a monetary amount for the core.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Money {
    /// The precise amount of the transaction in the corresponding currency.
    pub amount: Decimal,
    /// The currency of the transaction amount (e.g., USD, EUR).
    pub currency: Currency,
}

/// Unified transaction status codes used across all adapters.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Display, AsRefStr)]
pub enum TransactionStatus {
    /// Funds have been reserved on the customer's account.
    Authorized,
    /// Funds have been successfully debited from the customer's account.
    Captured,
    /// Awaiting further action (e.g., 3D Secure completion or asynchronous webhook).
    Pending,
    /// Transaction was declined by the gateway or the issuing bank.
    Declined,
    /// A technical failure occurred during processing (e.g., communication error).
    Failed,
    /// The original authorization was successfully canceled.
    Voided,
    /// Funds have been successfully returned to the customer.
    Refunded,
    /// The transaction is in a long-running state (e.g., ACH/SEPA transfer).
    Processing,
}

/// The source of funds for a core transaction (Vault-First principle).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PaymentSource {
    /// A canonical token representing a saved instrument (card, account, etc.).
    Token(String),
    /// Details for direct bank account transfers.
    BankAccount(BankAccountDetails),
    /// Other non-card, non-token sources (e.g., Wallet ID, QR code ID).
    Other(String),
}

/// Details about a bank account (for ACH/SEPA or similar transfers).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankAccountDetails {
    /// The customer's account number.
    pub account_number: String,
    /// The bank's routing number (e.g., ABA, SWIFT, Sort Code).
    /// See ADR-0001 for design decision.
    pub routing_number: String,
    /// The name of the bank, optional.
    pub bank_name: Option<String>,
}
