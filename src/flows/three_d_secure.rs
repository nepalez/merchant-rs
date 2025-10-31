use async_trait::async_trait;

use crate::Error;
use crate::types::{InternalPaymentMethod, Payment, PaymentToken};

/// Trait representing the 3D Secure authentication flow.
#[async_trait]
pub trait ThreeDSecure {
    #[allow(private_bounds)]
    async fn authenticate<Source: InternalPaymentMethod>(
        &self,
        payment: Payment<Source>,
    ) -> Result<PaymentToken<Source>, Error>;
}
