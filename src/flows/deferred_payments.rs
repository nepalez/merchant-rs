use async_trait::async_trait;
use rust_decimal::Decimal;

use crate::flows::change_authorization;
use crate::types::payments::PaymentMarker;
use crate::types::{
    CaptureAuthorized, InternalPaymentMethod, Recipients, StoredCredentialUsage, Transaction,
    TransactionId,
};
use crate::{Error, Gateway, MerchantInitiatedType};

trait CapturedAmount {}
impl CapturedAmount for CaptureAuthorized {}
impl CapturedAmount for Option<Decimal> {}

trait CapturedDistribution {}
impl CapturedDistribution for CaptureAuthorized {}
impl CapturedDistribution for Option<Recipients> {}

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
/// # Associated Types
///
/// * `PaymentMethod` - Payment method type constrained to internal methods (cards, tokens, etc.).
///   Determines which payment instruments can be used with this gateway for deferred payments.
#[async_trait]
#[allow(private_bounds)]
pub trait DeferredPayments: Gateway
where
    <<Self as Gateway>::Payment as PaymentMarker>::PaymentMethod: InternalPaymentMethod,
{
    type AuthorizationChanges: change_authorization::Sealed;
    type CapturedAmount: CapturedAmount;
    type CapturedDistribution: CapturedDistribution;

    /// Authorize payment and reserve funds without immediate capture.
    #[allow(private_interfaces)]
    async fn authorize(
        &self,
        payment: <Self as Gateway>::Payment,
        installments: <Self as Gateway>::Installments,
        merchant_initiated_type: Option<MerchantInitiatedType>,
        stored_credential_usage: Option<StoredCredentialUsage>,
    ) -> Result<Transaction, Error>;

    /// Capture previously authorized funds.
    ///
    /// Debits funds reserved during authorization. Supports partial captures
    /// and split payments to multiple recipients.
    ///
    /// # Parameters
    ///
    /// * `transaction_id` - ID of the previously authorized transaction
    /// * `amount` - Capture amount (None for full capture, Some for partial)
    /// * `recipients` - Distribution changes (None to keep original, Some for custom)
    ///
    /// # Returns
    ///
    /// Transaction record with capture status
    async fn capture(
        &self,
        transaction_id: TransactionId,
        captured_amount: Self::CapturedAmount,
        captured_distribution: Self::CapturedDistribution,
    ) -> Result<Transaction, Error>;
}
