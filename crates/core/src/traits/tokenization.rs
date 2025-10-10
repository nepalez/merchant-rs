use async_trait::async_trait;

use crate::error::{Error, Result};
use crate::traits::{Gateway, TokenizationSupport};
use crate::types::{
    CVV, CardDetails, CardExpiry, CardHolderName, CustomerId, PaymentToken, PrimaryAccountNumber,
};

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
#[derive(Debug)]
pub struct TokenizationRequest {
    /// The raw card details to be tokenized.
    pub card: CardDetails,
    /// Optional identifier for the customer to associate with the token.
    pub customer_id: Option<CustomerId>,
}

/// Response body after tokenization.
#[derive(Debug, Clone)]
pub struct TokenizationResponse {
    /// The resulting token string.
    pub token: PaymentToken,
    /// Indicates if the tokenization was successful.
    pub is_success: bool,
    /// Details of any error that occurred.
    pub error: Option<Error>,
}
