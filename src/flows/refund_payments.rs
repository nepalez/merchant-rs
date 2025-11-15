use async_trait::async_trait;
use rust_decimal::Decimal;

use crate::types::{Recipients, RedistributedAmount, Transaction, TransactionId};
use crate::{Error, Gateway};

trait Distribution {}
impl Distribution for () {}
impl Distribution for Option<Decimal> {}
impl Distribution for Option<Recipients> {}
impl Distribution for RedistributedAmount {}

/// Payment gateway trait for refund operations.
///
/// Supports returning funds to the customer for previously captured/charged transactions.
/// Refunds can be full (the entire transaction amount)
/// or partial (less than the original amount).
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
#[allow(private_bounds)]
pub trait RefundPayments: Gateway {
    type Distribution: Distribution;

    /// Refund a previously captured payment, either fully or partially.
    ///
    /// Returns funds to the customer's original payment method.
    /// The refund uses the same currency as the original transaction.
    ///
    /// # Parameters
    ///
    /// * `transaction_id` - ID of the captured transaction to refund
    /// * `distribution` - Refund distribution:
    ///   - `()`: Full refund with original distribution
    ///   - `Option<Decimal>`: Change refund amount only (partial refund)
    ///   - `Option<Recipients>`: Change recipients only
    ///   - `RedistributedAmount`: Change both amount and recipients
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
    /// * Total refunded amount cannot exceed the original transaction amount
    async fn refund(
        &self,
        transaction_id: TransactionId,
        distribution: Self::Distribution,
    ) -> Result<Transaction, Error>;
}
