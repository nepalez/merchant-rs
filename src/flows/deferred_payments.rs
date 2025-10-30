use async_trait::async_trait;
use rust_decimal::Decimal;

use crate::Error;
use crate::internal::InternalPaymentSource;
use crate::types::{Payment, Transaction, TransactionId};

/// Optional trait for payment gateways that support completing a two-step flow,
/// where the first step is an authorization and the second is a capture.
#[async_trait]
pub trait DeferredPayments {
    #[allow(private_bounds)]
    type Source: InternalPaymentSource;

    /// Immediately charge the payment.
    async fn authorize(&self, payment: Payment<Self::Source>) -> Result<Transaction, Error>;

    /// Confirms and debits the previously authorized funds.
    /// The `amount` parameter is used for partial captures.
    async fn capture(
        &self,
        transaction_id: TransactionId,
        amount: Option<Decimal>,
    ) -> Result<Transaction, Error>;
}
