use async_trait::async_trait;

use crate::types::payments::PaymentMarker;
use crate::types::{StorablePaymentMethod, VaultPaymentMethod};
use crate::{Error, Gateway};

/// Optional trait for payment gateways that support storing payment data in their vault.
/// The received token can be used later to either charge or authorize the payment.
///
/// This trait can be used to support recurring payments and stored payment methods.
#[async_trait]
#[allow(private_bounds, private_interfaces)]
pub trait StoreCredentials: Gateway
where
    <<Self as Gateway>::Payment as PaymentMarker>::PaymentMethod: VaultPaymentMethod,
{
    #[allow(private_bounds)]
    type StoredPaymentMethod: StorablePaymentMethod;

    /// Store payment method in gateway vault and receive a token
    async fn store(
        &self,
        payment_method: Self::StoredPaymentMethod,
    ) -> Result<<<Self as Gateway>::Payment as PaymentMarker>::PaymentMethod, Error>;

    /// Remove the stored payment method from gateway vault
    /// This operation is idempotent - removing an already deleted token does not return an error
    async fn unstore(
        &self,
        token: <<Self as Gateway>::Payment as PaymentMarker>::PaymentMethod,
    ) -> Result<(), Error>;
}
