use async_trait::async_trait;
use iso_currency::Currency;
use rust_decimal::Decimal;

use crate::types::{
    InternalPaymentMethod, MerchantInitiatedType, Recipients, StoredCredentialUsage, Transaction,
    TransactionIdempotenceKey,
};
use crate::{Error, Gateway};

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
/// * `Method` - Payment method type constrained to internal methods (cards, tokens, etc.)
#[async_trait]
#[allow(private_bounds)]
pub trait ImmediatePayments: Gateway
where
    <Self as Gateway>::PaymentMethod: InternalPaymentMethod,
{
    /// Immediately charge the payment (authorization and capture in one step).
    ///
    /// # Parameters
    ///
    /// * `payment` - Payment data containing method and transaction details.
    ///   Implementations should validate that `payment.recipients().as_ref().map(|r| r.validate_count(Self::MAX_ADDITIONAL_RECIPIENTS))`
    ///   returns Ok before processing.
    ///
    /// # Returns
    ///
    /// Transaction record with status indicating success or failure
    async fn charge(
        &self,
        payment_method: <Self as Gateway>::PaymentMethod,
        currency: Currency,
        total_amount: Decimal,
        recipients: Option<Recipients>,
        idempotence_key: TransactionIdempotenceKey,
        merchant_initiated_type: Option<MerchantInitiatedType>,
        stored_credential_usage: Option<StoredCredentialUsage>,
    ) -> Result<Transaction, Error>;
}
