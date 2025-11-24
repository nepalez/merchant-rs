use async_trait::async_trait;

use crate::types::{InternalPaymentMethod, ReversalReason, Transaction, TransactionId};
use crate::{Error, Gateway, PaymentMarker};

/// Payment gateway trait for reversing settled transactions.
///
/// Reverses a settled transaction by undoing the original transaction without creating
/// a new transaction. This differs from refund (which creates a new refund transaction)
/// and void (which cancels non-settled authorizations).
///
/// # When to Use
///
/// * ACH returns for erroneous transactions
/// * SEPA Direct Debit reversals within regulatory timeframes
/// * Correcting duplicate payments
/// * Fixing incorrect amounts or accounts
/// * Gateway-specific reversal requirements
///
/// # Requirements
///
/// Transaction must be settled. Reversal is typically only available for:
/// * ACH/bank transfers (within 5 banking days per Nacha rules)
/// * SEPA Core (within 5 banking days)
/// * SEPA B2B (within 2 business days)
///
/// # Comparison with Other Operations
///
/// * **vs. CancelPayments::void()**: Void cancels non-settled authorizations before capture.
///   Reversal undoes settled transactions.
/// * **vs. RefundPayments::refund()**: Refund creates a new transaction with potential fees.
///   Reversal undoes the original transaction as if it never occurred.
///
/// # Gateway Support
///
/// Not all gateways support explicit reversal operations. Major gateways with support:
/// * Checkout.com (automatic void or refund based on status)
/// * Adyen (reversal API for settled payments)
/// * Braintree (reverseTransaction for certain payment methods)
/// * Worldpay (authorization reversal)
/// * Stripe (transfer reversals for ACH)
///
/// # Examples
///
/// ```skip
/// // Reverse a duplicate payment
/// use merchant_rs::inputs::ReversalReason;
/// gateway.reverse(transaction_id, Some(ReversalReason::Duplicate)).await?;
///
/// // Reverse without specifying reason
/// gateway.reverse(transaction_id, None).await?;
/// ```
#[async_trait]
#[allow(private_bounds)]
pub trait ReversePayment: Gateway
where
    <<Self as Gateway>::Payment as PaymentMarker>::PaymentMethod: InternalPaymentMethod,
{
    /// Reverse a settled transaction.
    ///
    /// Undoes the original transaction without creating a new transaction.
    /// Available only for settled transactions and specific payment methods (ACH, SEPA, cards).
    ///
    /// # Parameters
    ///
    /// * `transaction_id` - ID of the settled transaction to reverse
    /// * `reason` - Optional semantic reason for the reversal (for audit trails)
    ///
    /// # Returns
    ///
    /// Transaction record representing the reversal operation
    ///
    /// # Timing Constraints
    ///
    /// * ACH: within 5 banking days after settlement
    /// * SEPA Core: within 5 banking days after settlement
    /// * SEPA B2B: within 2 business days after settlement
    /// * Cards: varies by gateway and network
    async fn reverse(
        &self,
        transaction_id: TransactionId,
        reason: Option<ReversalReason>,
    ) -> Result<Transaction, Error>;
}
