use async_trait::async_trait;

use crate::Error;
use crate::internal::InternalPaymentSource;
use crate::types::{Payment, Transaction};

/// Payment gateway trait for one-step payment flows.
///
/// Supports immediate charge transactions where authorization and capture occur in a single step.
/// Funds are debited from the customer's account immediately upon successful authorization.
///
/// # When to Use
///
/// * Digital goods and services (instant delivery)
/// * Low-value transactions where two-step flow is unnecessary
/// * Payment methods that don't support separate capture (some wallets, vouchers)
/// * Gateways that only provide combined auth+capture operations
///
/// # Type Parameter
///
/// * `Source` - Payment source type constrained to internal sources (cards, tokens, etc.)
#[async_trait]
pub trait ImmediatePayments {
    #[allow(private_bounds)]
    type Source: InternalPaymentSource;

    /// Immediately charge the payment (authorization and capture in one step).
    ///
    /// # Parameters
    ///
    /// * `payment` - Payment data containing source and transaction details
    ///
    /// # Returns
    ///
    /// Transaction record with status indicating success or failure
    async fn charge(&self, payment: Payment<Self::Source>) -> Result<Transaction, Error>;
}
