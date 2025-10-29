use async_trait::async_trait;

use crate::Error;
use crate::internal::PaymentSource;
use crate::types::{MerchantInitiatedType, Money, Payment, Transaction, TransactionIdempotenceKey};

/// Optional trait for payment gateways that support completing a one-step flow,
/// without the necessity to capture them later.
#[async_trait]
pub trait OneStepPayments {
    #[allow(private_bounds)]
    type Source: PaymentSource;

    /// Immediately charge the payment.
    async fn charge(&self, payment: Payment<Self::Source>) -> Result<Transaction, Error>;
}
