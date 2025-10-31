use async_trait::async_trait;

use crate::Error;
use crate::types::{Token, TokenizablePaymentMethod};

/// Optional trait for payment gateways that support tokenizing payment data.
/// The received token can be used later to either charge or authorize the payment.
///
/// This trait can be used to support 3D Secure payments.
#[async_trait]
pub trait TokenizeCredentials {
    #[allow(private_bounds)]
    type Method: TokenizablePaymentMethod;

    async fn tokenize(&self, method: Self::Method) -> Result<Token, Error>;
}
