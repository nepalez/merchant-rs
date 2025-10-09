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

// --- Financial Foundations ---

/// Canonical representation of a monetary amount for the core.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Money {
    /// The precise amount of the transaction in the corresponding currency.
    pub amount: Decimal,
    /// The currency of the transaction amount (e.g., USD, EUR).
    pub currency: Currency,
}

// --- Transaction Status ---

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

// --- Payment Source Details ---

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

// --- Core Transaction Request/Response Structures ---

/// TODO: Should be guarded by a feature flag (e.g., "standard-transactions").
/// Request body for authorizing a payment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationRequest {
    /// The monetary amount to be authorized.
    pub amount: Money,
    /// The source of the payment (must be a token or bank account details).
    pub source: PaymentSource,
    /// Unique ID provided by the merchant for tracing the transaction.
    pub merchant_reference_id: String,
    /// Optional identifier for the customer.
    pub customer_id: Option<String>,
    /// Opaque byte array representing arbitrary metadata. The format (e.g., JSON, XML)
    /// is interpreted by the Adapter, making the core format-agnostic.
    pub metadata: Option<Vec<u8>>,
}

/// TODO: Should be guarded by a feature flag (e.g., "standard-transactions").
/// Response body after an authorization attempt.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorizationResponse {
    /// Indicates if the transaction was successful (e.g., `Authorized` or `Captured`).
    pub is_success: bool,
    /// The unique transaction ID returned by the payment gateway.
    pub transaction_id: String,
    /// The canonical status of the transaction.
    pub status: TransactionStatus,
    /// Optional authorization code returned by the bank.
    pub authorization_code: Option<String>,
    /// Details of any error that occurred, even if the status is Declined.
    pub error: Option<Error>,
}

/// TODO: Should be guarded by a feature flag (e.g., "standard-transactions").
/// Request body for capturing a previously authorized payment.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureRequest {
    /// ID of the original authorization transaction returned by the gateway.
    pub authorization_id: String,
    /// The exact amount to capture. Must be less than or equal to the authorized amount.
    pub amount_to_capture: Money,
    /// Unique ID provided by the merchant for tracing the capture operation.
    pub merchant_reference_id: String,
}

/// TODO: Should be guarded by a feature flag (e.g., "standard-transactions").
/// Response body after a successful or failed capture.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptureResponse {
    /// Indicates if the operation was successful.
    pub is_success: bool,
    /// The new transaction ID for the capture operation.
    pub transaction_id: String,
    /// The canonical status (Should be Captured or Failed).
    pub status: TransactionStatus,
    /// The final amount successfully captured.
    pub captured_amount: Money,
    /// Details of any error that occurred.
    pub error: Option<Error>,
}

/// TODO: Should be guarded by a feature flag (e.g., "standard-transactions").
/// Request body for voiding (canceling) a pending authorization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoidRequest {
    /// ID of the authorization to void.
    pub authorization_id: String,
    /// Unique ID provided by the merchant for tracing the void operation.
    pub merchant_reference_id: String,
}

/// TODO: Should be guarded by a feature flag (e.g., "standard-transactions").
/// Response body after a successful or failed void operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoidResponse {
    /// Indicates if the operation was successful.
    pub is_success: bool,
    /// The transaction ID associated with the void operation.
    pub transaction_id: String,
    /// The canonical status (Should be Voided or Failed).
    pub status: TransactionStatus,
    /// Details of any error that occurred.
    pub error: Option<Error>,
}

/// TODO: Should be guarded by a feature flag (e.g., "standard-transactions").
/// Request body for initiating a refund.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundRequest {
    /// ID of the transaction to be refunded (usually a Capture ID).
    pub transaction_id: String,
    /// The exact amount to refund.
    pub amount_to_refund: Money,
    /// Optional reason for the refund, often required by the gateway.
    pub reason: Option<String>,
    /// Unique ID provided by the merchant for tracing the refund operation.
    pub merchant_reference_id: String,
}

/// TODO: Should be guarded by a feature flag (e.g., "standard-transactions").
/// Response body after a successful or failed refund.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefundResponse {
    /// Indicates if the operation was successful.
    pub is_success: bool,
    /// The unique ID returned by the gateway for the refund record.
    pub refund_id: String,
    /// The canonical status (Should be Refunded or Failed).
    pub status: TransactionStatus,
    /// The final amount successfully refunded.
    pub refunded_amount: Money,
    /// Details of any error that occurred.
    pub error: Option<Error>,
}

// --- Tokenization Request/Response (Optional Trait Support) ---

/// TODO: Should be guarded by a feature flag (e.g., "tokenization").
/// Details about a credit or debit card (Used ONLY for initial tokenization requests).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardDetails {
    /// The primary account number (PAN).
    pub number: String,
    /// The card's expiration month (1-12).
    pub expiry_month: u8,
    /// The card's expiration year (four digits).
    pub expiry_year: u16,
    /// The card verification value (CVV/CVC), optional for vaulted cards.
    pub cvv: Option<String>,
    /// The name of the cardholder, optional for most APIs.
    pub holder_name: Option<String>,
}

/// TODO: Should be guarded by a feature flag (e.g., "tokenization").
/// Request body to convert raw card data into a reusable token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenizationRequest {
    /// The raw card details to be tokenized.
    pub card: CardDetails,
    /// Optional identifier for the customer to associate with the token.
    pub customer_id: Option<String>,
}

/// TODO: Should be guarded by a feature flag (e.g., "tokenization").
/// Response body after tokenization.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenizationResponse {
    /// The resulting token string.
    pub token: String,
    /// Indicates if the tokenization was successful.
    pub is_success: bool,
    /// Details of any error that occurred.
    pub error: Option<Error>,
}
