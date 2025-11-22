use async_trait::async_trait;

use crate::Gateway;
use crate::types::{InternalPaymentMethod, TransactionId};
use crate::{Error, PaymentMarker};

/// Optional trait for payment gateways that support payment method verification.
///
/// Performs zero-dollar authorization to validate payment credentials without
/// charging or reserving funds. This is useful for:
///
/// * Validating cards before storing them in a vault
/// * Checking if a payment method is still valid
/// * Verifying billing address and CVV
/// * Fraud prevention and risk assessment
/// * Card updater services
///
/// ## Zero-Dollar Authorization
///
/// Also known as "account verification" or "$0 auth", this is a special authorization
/// request with amount = 0 that checks payment method validity without any financial impact.
///
/// Card networks (Visa, Mastercard) support and even recommend this approach over
/// $1 authorizations for validation purposes.
///
/// ## Use Cases
///
/// ### Adding Card to Wallet
/// ```skip
/// // Verify card before storing
/// let verification = gateway.verify_payment_method(credit_card).await?;
///
/// if verification.verified {
///     // Card is valid, safe to store
///     let token = gateway.store(credit_card).await?;
/// }
/// ```
///
/// ### Validating Stored Card
/// ```skip
/// // Check if stored card is still valid
/// let stored_card = get_stored_card_from_db();
/// let verification = gateway.verify_payment_method(stored_card).await?;
///
/// if !verification.verified {
///     // Card expired or invalid, ask customer to update
///     notify_customer_to_update_card();
/// }
/// ```
///
/// ## Verification Results
///
/// * **Success** (`Ok(TransactionId)`) - Payment method is valid
/// * **Failure** (`Err(Error)`) - Payment method failed verification
///
/// The adapter is responsible for interpreting gateway-specific response codes
/// (AVS, CVV, etc.) and deciding whether verification passed or failed.
///
/// ## Gateway Support
///
/// Not all gateways support zero-dollar authorization. Some may:
/// * Not support it at all
/// * Require special merchant account configuration
/// * Have different implementation approaches
#[async_trait]
#[allow(private_bounds, private_interfaces)]
pub trait VerifyAuthorization: Gateway
where
    <<Self as Gateway>::Payment as PaymentMarker>::PaymentMethod: InternalPaymentMethod,
{
    /// Verify a payment method without charging or reserving funds.
    ///
    /// Performs a zero-dollar authorization to validate the payment method.
    /// No funds are charged or reserved.
    ///
    /// # Parameters
    ///
    /// * `method` - Payment method to verify (e.g., CreditCard, BankPayment)
    ///
    /// # Returns
    ///
    /// * `Ok(TransactionId)` - Verification successful, returns transaction ID for audit trail
    /// * `Err(Error)` - Verification failed (invalid card, AVS mismatch, etc.)
    ///
    /// # Notes
    ///
    /// * The gateway adapter interprets verification response codes (AVS, CVV, etc.)
    /// * The verification transaction typically auto-voids immediately
    /// * Some payment methods may not support verification (e.g., cash vouchers)
    async fn verify_payment_method(
        &self,
        payment_method: <<Self as Gateway>::Payment as PaymentMarker>::PaymentMethod,
    ) -> Result<TransactionId, Error>;
}
