use async_trait::async_trait;

use crate::Error;
use crate::types::{Transaction, TransactionId};

/// Trait for payment gateways that support the return of funds to a customer.
#[async_trait]
pub trait RefundPayments {
    /// Refund a previously captured payment and return the refund transaction.
    async fn refund(&self, transaction_id: TransactionId) -> Result<Transaction, Error>;
}
