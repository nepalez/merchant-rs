use async_trait::async_trait;

use crate::Error;
use crate::Gateway;
use crate::types::{Transaction, TransactionId};

/// Base mandatory trait for all payment gateway adapters.
///
/// Every gateway adapter must implement this trait as the minimal requirement.
/// Provides transaction status lookup by ID, which is universally supported by all payment gateways.
///
/// # Purpose
///
/// * **Webhook verification**: Validate transaction status from webhook notifications
/// * **Reconciliation**: Match gateway transactions with internal records
/// * **Customer support**: Investigate transaction status for support inquiries
/// * **Status polling**: Check completion status for external payment flows
/// * **Audit compliance**: Retrieve transaction trail for audit purposes
///
/// # Implementation Note
///
/// This is the only mandatory trait - all payment flow traits (`ImmediatePayments`,
/// `DeferredPayments`, etc.) are optional based on gateway capabilities.
#[allow(private_bounds)]
#[async_trait]
pub trait CheckTransaction: Gateway {
    /// Retrieve transaction status and details by transaction ID.
    ///
    /// # Parameters
    ///
    /// * `transaction_id` - Unique transaction identifier assigned by the gateway
    ///
    /// # Returns
    ///
    /// Current transaction record with status, amount, and other details
    async fn status(&self, transaction_id: TransactionId) -> Result<Transaction, Error>;
}
