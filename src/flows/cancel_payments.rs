use async_trait::async_trait;

use crate::Error;
use crate::Gateway;
use crate::types::{Transaction, TransactionId};

/// Payment gateway trait for canceling/voiding transactions.
///
/// Supports canceling pending authorizations or reversing recent one-step charges before settlement.
/// Releases reserved funds without any charges to the customer.
///
/// # When to Use
///
/// * **Pending authorizations**: Cancel after `DeferredPayments::authorize()` before capture
/// * **Recent charges**: Reverse `ImmediatePayments::charge()` before settlement window closes
/// * **Out-of-stock**: Cancel order when item becomes unavailable
/// * **Fraud detection**: Cancel suspicious transaction before capture
///
/// # Requirements
///
/// Transaction must be in authorized (not captured) or recently charged (not settled) status.
/// Use `RefundPayments::refund()` for settled transactions.
///
/// # Settlement Windows
///
/// Most payment networks allow voids only within a limited timeframe:
/// * **Cards**: Same business day before settlement batch
/// * **ACH/Bank**: Before settlement date (1-3 business days)
/// * **Other**: Varies by payment method and network
#[async_trait]
pub trait CancelPayments: Gateway {
    /// Cancel/void a pending authorization or recent charge.
    ///
    /// Releases reserved funds for pending authorizations or reverses recent charges
    /// before settlement. This operation typically has no fees compared to refunds.
    ///
    /// # Parameters
    ///
    /// * `transaction_id` - ID of the transaction to cancel/void
    ///
    /// # Returns
    ///
    /// Transaction record representing the void operation
    async fn void(&self, transaction_id: TransactionId) -> Result<Transaction, Error>;
}
