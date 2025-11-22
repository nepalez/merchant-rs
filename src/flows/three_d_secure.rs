use async_trait::async_trait;
use iso_currency::Currency;
use rust_decimal::Decimal;

use crate::types::{
    InternalPaymentMethod, PaymentToken, StoredCredentialUsage, TransactionIdempotenceKey,
};
use crate::{Error, Gateway, MerchantInitiatedType};

/// Trait representing the 3D Secure authentication flow.
#[async_trait]
#[allow(private_bounds)]
pub trait ThreeDSecure: Gateway
where
    <Self as Gateway>::PaymentMethod: InternalPaymentMethod,
{
    #[allow(clippy::too_many_arguments)]
    async fn authenticate(
        &self,

        payment_method: <Self as Gateway>::PaymentMethod,
        currency: Currency,
        total_amount: Decimal,
        base_amount: Decimal,
        distribution: <Self as Gateway>::AmountDistribution,
        idempotence_key: TransactionIdempotenceKey,

        installments: <Self as Gateway>::Installments,
        merchant_initiated_type: Option<MerchantInitiatedType>,
        stored_credential_usage: Option<StoredCredentialUsage>,
    ) -> Result<PaymentToken<<Self as Gateway>::PaymentMethod>, Error>;
}
