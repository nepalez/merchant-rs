use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::error::{Error, Result};
use crate::traits::{Gateway, TokenizationSupport};

/// Trait for services that can convert raw payment details into a secure token.
#[async_trait]
pub trait Tokenizable
where
    Self: Gateway<TokenizationSupport = TokenizationRequired>,
{
    /// Converts raw payment details into a secure, opaque token string.
    async fn tokenize(&self, request: TokenizationRequest) -> Result<TokenizationResponse>;
}

/// Indicates that the adapter does not support refunding transactions.
/// Applies to gateways that do not allow refunds (e.g., some crypto payment processors).
pub struct TokenizationRequired;
impl TokenizationSupport for TokenizationRequired {}

/// Request body to convert raw card data into a reusable token.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenizationRequest {
    /// The raw card details to be tokenized.
    pub card: CardDetails,
    /// Optional identifier for the customer to associate with the token.
    pub customer_id: Option<String>,
}

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
