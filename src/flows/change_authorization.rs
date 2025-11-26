//! Authorization amount change flows.
//!
//! Provides traits for changing authorized amounts before capture in the two-step
//! payment flow. Different payment gateways use different approaches for specifying
//! authorization changes:
//!
//! * [`EditAuthorization`] - For gateways that accept the new total amount
//!   (Stripe, Adyen, Braintree, PayPal, Square)
//! * [`AdjustAuthorization`] - For gateways that accept delta/incremental amounts
//!   (Checkout.com, Worldpay, Authorize.Net, Computop, Cybersource)
//!
//! These traits represent alternative implementations of the same functionality,
//! allowing clients to constrain their code to specific gateway types based on
//! the change model they support.
//!
//! See [ADR-0013] for the architecture decision record.

use async_trait::async_trait;
use rust_decimal::Decimal;

use crate::flows::DeferredPayments;
use crate::types::payments::PaymentMarker;
use crate::types::{InternalPaymentMethod, Transaction, TransactionId};
use crate::{Error, Gateway};

/// Sealed trait for authorization change model marker types.
///
/// This trait is sealed to prevent external implementations. Only the three
/// marker types defined in this module can implement it: [`ChangesNotSupported`],
/// [`ChangesByTotal`], and [`ChangesByDelta`].
///
/// The sealed pattern ensures that gateway implementations can only use these
/// predefined change models, preventing incompatible trait implementations
/// at compile time.
pub(crate) trait Sealed {}

/// Marker type indicating that authorization changes are not supported.
///
/// This is the default for gateways implementing [`DeferredPayments`].
/// Gateways with this marker type cannot implement [`EditAuthorization`] or [`AdjustAuthorization`].
pub struct ChangesNotSupported;
impl Sealed for ChangesNotSupported {}

/// Marker type for gateways that accept new total amounts for authorization changes.
///
/// Gateways with this marker type implement [`EditAuthorization`], which accepts the
/// complete new authorization amount. The gateway calculates whether to increment
/// or decrement based on the difference from the current amount.
///
/// # Examples of Gateways
///
/// Stripe, Adyen, Braintree, PayPal, Square
pub struct ChangesByTotal;
impl Sealed for ChangesByTotal {}

/// Marker type for gateways that accept delta amounts for authorization changes.
///
/// Gateways with this marker type implement [`AdjustAuthorization`], which provides
/// separate methods for incrementing and decrementing authorization by specifying
/// the amount to add or release.
///
/// # Examples of Gateways
///
/// Checkout.com, Worldpay, Authorize.Net, Computop, Cybersource
pub struct ChangesByDelta;
impl Sealed for ChangesByDelta {}

/// Optional trait for payment gateways that support editing authorization
/// to a new total amount in the two-step flow.
///
/// Used by gateways that accept the complete new authorization amount.
/// The gateway calculates whether to increment or decrement based on
/// the difference from the current authorized amount.
///
/// # Supported Gateways
///
/// Stripe, Adyen, Braintree, PayPal, Square
///
/// # Important Limitations
///
/// * **Split payments**: Most gateways do NOT support changing payment destinations
///   (splits) during authorization changes. The split configuration from the original
///   authorization remains in effect.
/// * **Checkout.com**: Explicitly prohibits incremental authorization when
///   `amount_allocations` (splits) are present.
/// * **Use case**: Primarily for single-destination payments (Platform) or when
///   split ratios can remain unchanged.
///
/// # Typical Limits
///
/// * Maximum increments: 10 per authorization (Stripe)
/// * Amount cap: 115-500% of original or fixed limit ($75-500)
/// * Card networks: Primarily Visa, Mastercard, Amex, Discover
#[async_trait]
#[allow(private_bounds)]
pub trait EditAuthorization: DeferredPayments<AuthorizationChanges = ChangesByTotal>
where
    <<Self as Gateway>::Payment as PaymentMarker>::PaymentMethod: InternalPaymentMethod,
{
    /// Edit authorization to a new total amount.
    ///
    /// Changes the authorized amount before capture by specifying the complete
    /// new total. The gateway determines whether to increment or decrement.
    ///
    /// # Parameters
    ///
    /// * `transaction_id` - ID of the previously authorized transaction
    /// * `new_amount` - New total amount to authorize (must differ from the current amount)
    ///
    /// # Returns
    ///
    /// Updated transaction record with the new authorized amount
    ///
    /// # Errors
    ///
    /// * Gateway does not support amount changes
    /// * New amount exceeds gateway/network limits
    /// * Changes not allowed for split payments (gateway-specific)
    /// * Maximum number of changes exceeded
    async fn edit_authorization(
        &self,
        transaction_id: TransactionId,
        new_amount: Decimal,
    ) -> Result<Transaction, Error>;
}

/// Optional trait for payment gateways that support incremental/decremental
/// authorization changes via delta amounts.
///
/// Used by gateways that accept additional amounts rather than the new total.
/// Provides separate methods for increment and decrement operations.
///
/// # Supported Gateways
///
/// Checkout.com, Worldpay, Authorize.Net, Computop, Cybersource
///
/// # Important Limitations
///
/// * **Split payments**: Most gateways do NOT support changing payment destinations
///   (splits) during authorization changes. The split configuration from the original
///   authorization remains in effect.
/// * **Checkout.com**: Explicitly prohibits incremental authorization when
///   `amount_allocations` (splits) are present.
/// * **Use case**: Primarily for single-destination payments (Platform) or when
///   split ratios can remain unchanged.
///
/// # Typical Limits
///
/// * Maximum increments: Varies by gateway
/// * Amount cap: 115-500% of original or fixed limit ($75-500)
/// * Card networks: Primarily Visa, Mastercard, Amex, Discover
#[async_trait]
#[allow(private_bounds)]
pub trait AdjustAuthorization: DeferredPayments<AuthorizationChanges = ChangesByDelta>
where
    <<Self as Gateway>::Payment as PaymentMarker>::PaymentMethod: InternalPaymentMethod,
{
    /// Increment authorization by adding an amount.
    ///
    /// Increases the authorized amount before capture by specifying how much
    /// to add to the current authorization.
    ///
    /// # Parameters
    ///
    /// * `transaction_id` - ID of the previously authorized transaction
    /// * `additional_amount` - Amount to add to the current authorization (must be positive)
    ///
    /// # Returns
    ///
    /// Updated transaction record with the increased authorized amount
    ///
    /// # Errors
    ///
    /// * Gateway does not support authorization increment
    /// * Additional amount would exceed gateway/network limits
    /// * Increment not allowed for split payments (gateway-specific)
    /// * Maximum number of adjustments exceeded
    async fn increment_authorization(
        &self,
        transaction_id: TransactionId,
        additional_amount: Decimal,
    ) -> Result<Transaction, Error>;

    /// Decrement authorization by releasing a portion of reserved funds.
    ///
    /// Decreases the authorized amount before capture by specifying how much
    /// to release from the current authorization.
    ///
    /// # Parameters
    ///
    /// * `transaction_id` - ID of the previously authorized transaction
    /// * `amount_to_release` - Amount to release from current authorization (must be positive)
    ///
    /// # Returns
    ///
    /// Updated transaction record with the decreased authorized amount
    ///
    /// # Errors
    ///
    /// * Gateway does not support authorization decrement
    /// * Amount to release exceeds the current authorized amount
    /// * Decrement not allowed for split payments (gateway-specific)
    async fn decrement_authorization(
        &self,
        transaction_id: TransactionId,
        amount_to_release: Decimal,
    ) -> Result<Transaction, Error>;
}
