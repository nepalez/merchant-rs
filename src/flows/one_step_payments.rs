use async_trait::async_trait;

use crate::internal::PaymentSource;
use crate::types::{Payment, Transaction};
use crate::Error;

/// Optional trait for payment gateways that support completing a one-step flow,
/// without the necessity to capture them later.
#[async_trait]
pub trait OneStepPayments {
    #[allow(private_bounds)]
    type Source: PaymentSource;

    /// Immediately charge the payment.
    async fn charge(&self, payment: Payment<Self::Source>) -> Result<Transaction, Error>;
}
