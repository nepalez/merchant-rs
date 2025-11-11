use async_trait::async_trait;

use crate::Error;
use crate::Gateway;
use crate::types::{StorablePaymentMethod, Token};

/// Optional trait for payment gateways that support storing payment data in their vault.
/// The received token can be used later to either charge or authorize the payment.
///
/// This trait can be used to support recurring payments and stored payment methods.
#[async_trait]
pub trait StoreCredentials: Gateway {
    #[allow(private_bounds)]
    type Method: StorablePaymentMethod;

    /// Store payment method in gateway vault and receive a token
    async fn store(&self, method: Self::Method) -> Result<Token, Error>;

    /// Remove the stored payment method from gateway vault
    /// This operation is idempotent - removing an already deleted token does not return an error
    async fn unstore(&self, token: Token) -> Result<(), Error>;
}
