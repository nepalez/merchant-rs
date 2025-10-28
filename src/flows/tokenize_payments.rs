use async_trait::async_trait;

use crate::Error;
use crate::types::{CreditCard, Token};

/// Optional trait for payment gateways that support tokenizing a payment
/// by exchanging the payment details for a secure token.
///
/// Later the token can be used to charge the payment via `PaymentSource::TokenizedPayment`.
/// This method can be used to support 3D Secure payments.
#[async_trait]
pub trait TokenizePayments {
    fn tokenize(&self, card: CreditCard) -> Result<Token, Error>;
}
