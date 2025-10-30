use async_trait::async_trait;

use crate::Error;
use crate::types::{Payment, Transaction, payment_token::Source as PaymentSource};

/// Optional trait for payment gateways that support completing a one-step flow,
/// without the necessity to capture them later.
#[async_trait]
pub trait ImmediatePayments {
    #[allow(private_bounds)]
    type Source: PaymentSource;

    /// Immediately charge the payment.
    async fn charge(&self, payment: Payment<Self::Source>) -> Result<Transaction, Error>;
}
