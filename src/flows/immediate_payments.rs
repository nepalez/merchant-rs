use async_trait::async_trait;
use iso_currency::Currency;
use rust_decimal::Decimal;

use crate::types::{
    DistributedAmount, InternalPaymentMethod, MerchantInitiatedType, StoredCredentialUsage,
    Transaction, TransactionIdempotenceKey,
};
use crate::{Error, Gateway};

trait Amount {}
impl Amount for Decimal {}
impl Amount for DistributedAmount {}

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
/// # Type Parameters
///
/// * `Amount` - Payment amount type (Decimal or DistributedAmount for split payments)
#[async_trait]
#[allow(private_bounds)]
pub trait ImmediatePayments: Gateway
where
    <Self as Gateway>::PaymentMethod: InternalPaymentMethod,
{
    type Amount: Amount;

    /// Immediately charge the payment (authorization and capture in one step).
    ///
    /// # Parameters
    ///
    /// * `amount` - Payment amount, either simple Decimal or DistributedAmount with recipients
    ///
    /// # Returns
    ///
    /// Transaction record with status indicating success or failure
    async fn charge(
        &self,
        payment_method: <Self as Gateway>::PaymentMethod,
        amount: Self::Amount,
        currency: Currency,
        idempotence_key: TransactionIdempotenceKey,
        merchant_initiated_type: Option<MerchantInitiatedType>,
        stored_credential_usage: Option<StoredCredentialUsage>,
    ) -> Result<Transaction, Error>;
}
