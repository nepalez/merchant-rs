use async_trait::async_trait;

use crate::Error;
use crate::types::{Payment, Transaction};

/// Optional trait for payment gateways that support completing a one-step flow,
/// without the necessity to capture them later.
#[async_trait]
pub trait OneStepPayments {
    /// Immediately charge the payment.
    async fn charge(&self, payment: Payment) -> Result<Transaction, Error>;
}
