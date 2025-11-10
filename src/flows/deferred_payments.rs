use async_trait::async_trait;

use crate::Error;
use crate::flows::change_authorization;
use crate::types::{Destinations, InternalPaymentMethod, Payment, Transaction, TransactionId};

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
///
/// * `AuthorizationChanges` - Marker type indicating which authorization change model
///   the gateway supports. Gateway implementations should set this to:
///   - `change_authorization::ChangesNotSupported` (default) - no authorization changes supported
///   - `change_authorization::ChangesByTotal` - implements [`EditAuthorization`] trait
///   - `change_authorization::ChangesByDelta` - implements [`AdjustAuthorization`] trait
#[async_trait]
pub trait DeferredPayments {
    #[allow(private_bounds)]
    type PaymentMethod: InternalPaymentMethod;
    #[allow(private_bounds)]
    type AuthorizationChanges: change_authorization::Sealed;

    /// Authorize payment and reserve funds without immediate capture.
    ///
    /// Validates the payment method and reserves the specified amount on the customer's account.
    /// Funds remain reserved but not debited until `capture()` is called.
    ///
    /// # Parameters
    ///
    /// * `payment` - Payment data containing method and transaction details
    ///
    /// # Returns
    ///
    /// Transaction record with authorization status
    async fn authorize(&self, payment: Payment<Self::PaymentMethod>) -> Result<Transaction, Error>;

    /// Capture previously authorized funds.
    ///
    /// Debits funds reserved during authorization. Supports partial captures
    /// and split payments to multiple recipients.
    ///
    /// # Parameters
    ///
    /// * `transaction_id` - ID of the previously authorized transaction
    /// * `destinations` - Optional payment destinations:
    ///   - `None`: Capture using the destinations specified during authorization
    ///   - `Some(destinations)`: Override authorization destinations (partial capture
    ///     or split modification). Not all gateways support destination override:
    ///     * **Adyen, Checkout.com**: Support full override
    ///     * **Stripe, PayPal, Braintree**: Only amount can be changed (partial capture),
    ///       split configuration remains from authorization
    ///
    /// # Returns
    ///
    /// Transaction record with capture status
    async fn capture(
        &self,
        transaction_id: TransactionId,
        destinations: Option<Destinations>,
    ) -> Result<Transaction, Error>;
}
