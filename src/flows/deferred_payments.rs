use async_trait::async_trait;
use rust_decimal::Decimal;

use crate::Error;
use crate::internal::InternalPaymentSource;
use crate::types::{Payment, Transaction, TransactionId};

/// Payment gateway trait for two-step payment flows.
///
/// Supports deferred payment processing where authorization and capture are separate operations.
/// Authorization reserves funds on the customer's account, capture actually debits them.
///
/// # When to Use
///
/// * Physical goods (authorize at checkout, capture at shipment)
/// * Services with delayed delivery (hotels, car rentals)
/// * Split shipments (authorize total, capture per shipment)
/// * Risk management (review transactions before capture)
/// * Partial captures (authorize full amount, capture less)
///
/// # Flow
///
/// 1. **Authorize**: Reserve funds, validate payment method
/// 2. **Capture**: Debit reserved funds (full or partial)
/// 3. **Void**: Cancel authorization before capture (via `CancelPayments` trait)
///
/// # Type Parameter
///
/// * `Source` - Payment source type constrained to internal sources (cards, tokens, etc.)
#[async_trait]
pub trait DeferredPayments {
    #[allow(private_bounds)]
    type Source: InternalPaymentSource;

    /// Authorize payment and reserve funds without immediate capture.
    ///
    /// Validates the payment method and reserves the specified amount on the customer's account.
    /// Funds remain reserved but not debited until `capture()` is called.
    ///
    /// # Parameters
    ///
    /// * `payment` - Payment data containing source and transaction details
    ///
    /// # Returns
    ///
    /// Transaction record with authorization status
    async fn authorize(&self, payment: Payment<Self::Source>) -> Result<Transaction, Error>;

    /// Capture previously authorized funds.
    ///
    /// Debits funds that were reserved during authorization. Supports partial captures
    /// by specifying an amount less than the authorized amount.
    ///
    /// # Parameters
    ///
    /// * `transaction_id` - ID of the previously authorized transaction
    /// * `amount` - Optional amount for partial capture (None captures full authorized amount)
    ///
    /// # Returns
    ///
    /// Transaction record with capture status
    async fn capture(
        &self,
        transaction_id: TransactionId,
        amount: Option<Decimal>,
    ) -> Result<Transaction, Error>;
}
