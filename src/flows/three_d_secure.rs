use async_trait::async_trait;

use crate::types::{InternalPaymentMethod, PaymentToken, StoredCredentialUsage};
use crate::{Error, Gateway, MerchantInitiatedType, PaymentMarker};

/// Trait representing the 3D Secure authentication flow.
#[async_trait]
#[allow(private_bounds)]
pub trait ThreeDSecure: Gateway
where
    <<Self as Gateway>::Payment as PaymentMarker>::PaymentMethod: InternalPaymentMethod,
{
    #[allow(private_interfaces)]
    async fn authenticate(
        &self,
        payment: <Self as Gateway>::Payment,
        installments: <Self as Gateway>::Installments,
        merchant_initiated_type: Option<MerchantInitiatedType>,
        stored_credential_usage: Option<StoredCredentialUsage>,
    ) -> Result<PaymentToken<<<Self as Gateway>::Payment as PaymentMarker>::PaymentMethod>, Error>;
}
