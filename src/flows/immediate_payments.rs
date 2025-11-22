use async_trait::async_trait;
use iso_currency::Currency;
use rust_decimal::Decimal;

use crate::types::{
    InternalPaymentMethod, StoredCredentialUsage, Transaction, TransactionIdempotenceKey,
};
use crate::{Error, Gateway, MerchantInitiatedType};

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
    /// * `total_amount` - Total payment amount
    /// * `base_amount` - Amount going to the platform
    /// * `recipients` - Amount distribution to recipients (None if no distribution)
    /// * `installments` - Installment payment options
    ///
    /// # Returns
    ///
    /// Transaction record with status indicating success or failure
    #[allow(clippy::too_many_arguments)]
    async fn charge(
        &self,

        payment_method: <Self as Gateway>::PaymentMethod,
        currency: Currency,
        total_amount: Decimal,
        base_amount: Decimal,
        distribution: <Self as Gateway>::AmountDistribution,
        idempotence_key: TransactionIdempotenceKey,

        installments: <Self as Gateway>::Installments,
        merchant_initiated_type: Option<MerchantInitiatedType>,
        stored_credential_usage: Option<StoredCredentialUsage>,
    ) -> Result<Transaction, Error>;
}
