use async_trait::async_trait;

use crate::Error;
use crate::types::{
    InternalPaymentMethod, Money, RecurrentPayment, Subscription, SubscriptionId,
    SubscriptionInterval,
};

/// Payment gateway trait for recurrent payment subscriptions.
///
/// Supports creating and managing recurrent billing subscriptions where customers
/// are automatically charged at regular intervals.
///
/// # When to Use
///
/// * Subscription-based services (SaaS, streaming, memberships)
/// * Recurrent donations
/// * Installment payments
/// * Auto-renewal services
///
/// # Type Parameter
///
/// * `Method` - Payment method type constrained to internal methods (cards, tokens, etc.)
#[async_trait]
pub trait RecurrentPayments {
    #[allow(private_bounds)]
    type Method: InternalPaymentMethod;

    /// Create a new recurrent payment subscription.
    ///
    /// # Parameters
    ///
    /// * `payment` - Recurrent payment data containing method, amount, interval, and idempotence key
    ///
    /// # Returns
    ///
    /// Subscription record with ID, status, and billing schedule
    async fn create_subscription(
        &self,
        payment: RecurrentPayment<Self::Method>,
    ) -> Result<Subscription, Error>;

    /// Cancel an existing subscription.
    ///
    /// Cancels the subscription and stops future billing. Some gateways may allow
    /// cancellation at the period end, others cancel immediately.
    ///
    /// # Parameters
    ///
    /// * `subscription_id` - ID of the subscription to cancel
    ///
    /// # Returns
    ///
    /// Updated subscription record with a canceled status
    async fn cancel_subscription(
        &self,
        subscription_id: SubscriptionId,
    ) -> Result<Subscription, Error>;

    /// Update an existing subscription.
    ///
    /// Updates subscription parameters. Not all gateways support all updates.
    /// Interval changes may not be supported by some gateways.
    ///
    /// # Parameters
    ///
    /// * `subscription_id` - ID of the subscription to update
    /// * `amount` - New amount (None to keep current)
    /// * `interval` - New interval (None to keep current, may not be supported)
    ///
    /// # Returns
    ///
    /// Updated subscription record
    ///
    /// # Default Implementation
    ///
    /// Returns `Error::NotSupported` by default. Override if gateway supports updates.
    async fn update_subscription(
        &self,
        _subscription_id: SubscriptionId,
        _amount: Option<Money>,
        _interval: Option<SubscriptionInterval>,
    ) -> Result<Subscription, Error> {
        Err(Error::NotSupported(
            "Subscription updates are not supported by this gateway".into(),
        ))
    }

    /// Get the current status of a subscription.
    ///
    /// # Parameters
    ///
    /// * `subscription_id` - ID of the subscription to query
    ///
    /// # Returns
    ///
    /// Current subscription record
    async fn subscription_status(
        &self,
        subscription_id: SubscriptionId,
    ) -> Result<Subscription, Error>;

    /// Pause a subscription temporarily.
    ///
    /// Temporarily stops billing without canceling the subscription.
    /// Not all gateways support this operation.
    ///
    /// # Parameters
    ///
    /// * `subscription_id` - ID of the subscription to pause
    ///
    /// # Returns
    ///
    /// Updated subscription record with paused status
    ///
    /// # Default Implementation
    ///
    /// Returns `Error::NotSupported` by default. Override if gateway supports pausing.
    async fn pause_subscription(
        &self,
        _subscription_id: SubscriptionId,
    ) -> Result<Subscription, Error> {
        Err(Error::NotSupported(
            "Pausing subscriptions is not supported by this gateway".into(),
        ))
    }

    /// Resume a paused subscription.
    ///
    /// Resumes billing on a previously paused subscription.
    /// Not all gateways support this operation.
    ///
    /// # Parameters
    ///
    /// * `subscription_id` - ID of the subscription to resume
    ///
    /// # Returns
    ///
    /// Updated subscription record with active status
    ///
    /// # Default Implementation
    ///
    /// Returns `Error::NotSupported` by default. Override if gateway supports resuming.
    async fn resume_subscription(
        &self,
        _subscription_id: SubscriptionId,
    ) -> Result<Subscription, Error> {
        Err(Error::NotSupported(
            "Resuming subscriptions is not supported by this gateway".into(),
        ))
    }
}
