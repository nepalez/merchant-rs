use async_trait::async_trait;

use crate::Error;
use crate::types::{Transaction, TransactionId};

/// Payment gateway trait for refund operations.
///
/// Supports returning funds to the customer for previously captured/charged transactions.
/// Refunds can be full (entire transaction amount) or partial (less than original amount).
///
/// # When to Use
///
/// * Product returns or cancellations
/// * Service disputes or customer complaints
/// * Pricing errors or overcharges
/// * Failed service delivery
///
/// # Requirements
///
/// Transaction must be in captured/settled status. Use `CancelPayments::void()` for
/// canceling pending authorizations before capture.
#[async_trait]
pub trait RefundPayments {
    /// Refund a previously captured payment.
    ///
    /// Returns funds to the customer's original payment method. The refund amount
    /// is typically the full transaction amount, though some gateways support partial refunds.
    ///
    /// # Parameters
    ///
    /// * `transaction_id` - ID of the captured transaction to refund
    ///
    /// # Returns
    ///
    /// Transaction record representing the refund operation
    async fn refund(&self, transaction_id: TransactionId) -> Result<Transaction, Error>;
}
