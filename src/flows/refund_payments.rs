use async_trait::async_trait;
use rust_decimal::Decimal;

use crate::Error;
use crate::Gateway;
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
/// * Partial product returns (e.g., returning 2 of 3 items)
/// * Shipping cost adjustments
///
/// # Requirements
///
/// Transaction must be in captured/settled status. Use `CancelPayments::void()` for
/// canceling pending authorizations before capture.
///
/// # Partial Refunds
///
/// Many gateways support multiple partial refunds for a single transaction, as long as
/// the cumulative refund amount does not exceed the original transaction amount.
///
/// # Examples
///
/// ```skip
/// // Full refund
/// gateway.refund(transaction_id, None).await?;
///
/// // Partial refund - return $20 from $100 transaction
/// use rust_decimal::Decimal;
/// gateway.refund(transaction_id, Some(Decimal::from(20))).await?;
/// ```
#[async_trait]
pub trait RefundPayments: Gateway {
    /// Refund a previously captured payment, either fully or partially.
    ///
    /// Returns funds to the customer's original payment method.
    /// The refund uses the same currency as the original transaction.
    ///
    /// # Parameters
    ///
    /// * `transaction_id` - ID of the captured transaction to refund
    /// * `amount` - Optional refund amount:
    ///   - `None` - Full refund (entire transaction amount)
    ///   - `Some(decimal)` - Partial refund for the specified amount
    ///
    /// # Returns
    ///
    /// Transaction record representing the refund operation
    ///
    /// # Notes
    ///
    /// * Partial refund amount must not exceed the remaining refundable balance
    /// * Currency is inherited from the original transaction
    /// * Multiple partial refunds may be performed on the same transaction
    /// * Total refunded amount cannot exceed original transaction amount
    async fn refund(
        &self,
        transaction_id: TransactionId,
        amount: Option<Decimal>,
    ) -> Result<Transaction, Error>;
}
