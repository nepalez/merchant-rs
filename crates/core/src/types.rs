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
mod secret_string;
mod transaction_id;

use crate::error::{Error, ErrorCode, Result};
use secret_string::SecretString;
use std::fmt;
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

/// Sealed trait for types that require input sanitization before validation.
///
/// Sanitization typically involves:
/// - Removing allowed separators (spaces, hyphens, underscores)
/// - Normalizing whitespace
/// - Filtering invalid characters
///
/// # Default Implementation
///
/// The default implementation performs no sanitization (identity function).
/// This is appropriate for types that accept input as-is.
pub(crate) trait Sanitized {
    /// Sanitizes the input string, returning the cleaned version.
    #[inline]
    fn sanitize(input: String) -> Result<String> {
        Ok(input)
    }
}

/// Sealed trait for types that require domain validation of their input.
///
/// Validation occurs on the *sanitized* input and typically includes:
/// - Length checks (min/max/exact)
/// - Format validation (digits only, alphanumeric, etc.)
/// - Checksum validation (Luhn, etc.)
/// - Domain-specific rules
///
/// # Default Implementation
///
/// The default implementation accepts all input as valid.
/// This is appropriate for types with no validation requirements.
pub(crate) trait Validated {
    /// Validates the sanitized input string.
    #[inline]
    fn validate(_input: &str) -> Result<()> {
        Ok(())
    }
}

/// Sealed trait for newtype wrappers that can be constructed from validated strings.
///
/// This trait combines `Sanitized` and `Validated` to provide a complete
/// construction pipeline for secure wrapper types. It handles the three-step
/// process: sanitize → validate → wrap.
pub(crate) trait SafeWrapper: Sanitized + Validated + Sized {
    /// The inner type that can be constructed from a string.
    type Inner: From<String>;

    /// Wraps the inner value (obtained from a sanitized and validated string) into `Self`.
    /// This is typically a trivial newtype constructor: `Self(inner)`.
    fn wrap(inner: Self::Inner) -> Self;

    /// Constructs an instance of Self from a raw input string.
    #[inline]
    fn try_from_string(input: String) -> Result<Self> {
        let sanitized = Self::sanitize(input)?;
        Self::validate(&sanitized)?;
        Ok(Self::wrap(sanitized.into()))
    }
}
