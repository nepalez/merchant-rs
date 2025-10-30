use async_trait::async_trait;

use crate::Error;
use crate::internal::TokenizablePaymentSource;
use crate::types::Token;

/// Optional trait for payment gateways that support tokenizing payment data.
/// The received token can be used later to either charge or authorize the payment.
///
/// This trait can be used to support 3D Secure payments.
#[async_trait]
pub trait TokenizePaymentSources {
    #[allow(private_bounds)]
    type Source: TokenizablePaymentSource;

    async fn tokenize(&self, source: Self::Source) -> Result<Token, Error>;
}
