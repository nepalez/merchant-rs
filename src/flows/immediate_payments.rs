use async_trait::async_trait;

use crate::types::payments::PaymentMarker;
use crate::types::{InternalPaymentMethod, StoredCredentialUsage, Transaction};
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
    <<Self as Gateway>::Payment as PaymentMarker>::PaymentMethod: InternalPaymentMethod,
{
    /// Immediately charge the payment (authorization and capture in one step).
    #[allow(private_interfaces)]
    async fn charge(
        &self,
        payment: <Self as Gateway>::Payment,
        installments: <Self as Gateway>::Installments,
        merchant_initiated_type: Option<MerchantInitiatedType>,
        stored_credential_usage: Option<StoredCredentialUsage>,
    ) -> Result<Transaction, Error>;
}
