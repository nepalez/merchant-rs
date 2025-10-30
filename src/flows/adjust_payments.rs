use async_trait::async_trait;
use rust_decimal::Decimal;

use crate::Error;
use crate::flows::DeferredPayments;
use crate::types::{Transaction, TransactionId};

/// Optional trait for payment gateways that support adjusting a payment
/// after authorization (either incrementing or decrementing its amount)
/// of the two-step flow.
#[async_trait]
pub trait AdjustPayments: DeferredPayments {
    async fn adjust_payment(
        &self,
        transaction_id: TransactionId,
        amount: Decimal,
    ) -> Result<Transaction, Error>;
}
