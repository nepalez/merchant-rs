use async_trait::async_trait;
use rust_decimal::Decimal;

use crate::types::payments::PaymentMarker;
use crate::types::{
    InternalPaymentMethod, Recipients, Subscription, SubscriptionId, SubscriptionInterval,
};
use crate::{Error, Gateway};

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
#[async_trait]
#[allow(private_bounds)]
pub trait RecurrentPayments: Gateway
where
    <<Self as Gateway>::Payment as PaymentMarker>::PaymentMethod: InternalPaymentMethod,
{
    /// Create a new recurrent payment subscription.
    #[allow(private_interfaces)]
    async fn create_subscription(
        &self,
        payment: <Self as Gateway>::Payment,
        interval: SubscriptionInterval,
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
    async fn cancel_subscription(&self, subscription_id: SubscriptionId) -> Result<(), Error>;

    /// Get the current subscription details.
    ///
    /// # Parameters
    ///
    /// * `subscription_id` - ID of the subscription to query
    ///
    /// # Returns
    ///
    /// Current subscription record
    async fn get_subscription(
        &self,
        subscription_id: SubscriptionId,
    ) -> Result<Subscription, Error>;
}

/// Optional trait for gateways that support pausing and resuming subscriptions.
///
/// Many payment gateways allow temporarily pausing a subscription without canceling it,
/// which is useful for customers who want to temporarily suspend service.
///
/// # Examples of Supporting Gateways
///
/// * Stripe, Razorpay, Square, Xendit, Conekta, Paddle, Chargebee
///
/// # Examples of Non-Supporting Gateways
///
/// * Authorize.Net (requires cancel + recreate workaround)
#[async_trait]
#[allow(private_bounds)]
pub trait PauseSubscriptions: RecurrentPayments
where
    <<Self as Gateway>::Payment as PaymentMarker>::PaymentMethod: InternalPaymentMethod,
{
    /// Pause a subscription temporarily.
    ///
    /// Temporarily stops billing without canceling the subscription.
    ///
    /// # Parameters
    ///
    /// * `subscription_id` - ID of the subscription to pause
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success
    async fn pause_subscription(&self, subscription_id: SubscriptionId) -> Result<(), Error>;

    /// Resume a paused subscription.
    ///
    /// Resumes billing on a previously paused subscription.
    ///
    /// # Parameters
    ///
    /// * `subscription_id` - ID of the subscription to resume
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success
    async fn resume_subscription(&self, subscription_id: SubscriptionId) -> Result<(), Error>;
}

/// Optional trait for gateways that support editing subscription amount.
///
/// Most gateways with subscription support allow changing the recurring payment amount.
///
/// # Examples of Supporting Gateways
///
/// * Stripe, Razorpay, MercadoPago, GoCardless, Braintree, Paddle, Chargebee
#[async_trait]
#[allow(private_bounds)]
pub trait EditSubscriptionAmount: RecurrentPayments
where
    <<Self as Gateway>::Payment as PaymentMarker>::PaymentMethod: InternalPaymentMethod,
{
    /// Edit the amount of an existing subscription.
    ///
    /// # Parameters
    ///
    /// * `subscription_id` - ID of the subscription to edit
    /// * `amount` - New subscription amount
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success
    async fn edit_subscription_amount(
        &self,
        subscription_id: SubscriptionId,
        total_amount: Decimal,
    ) -> Result<(), Error>;
}

/// Optional trait for gateways that support editing subscription payment recipients.
///
/// Only applicable to gateways with split payment support (Gateway::MAX_ADDITIONAL_RECIPIENTS > 0).
///
/// # Examples of Supporting Gateways
///
/// * Gateways with split payment capabilities
#[async_trait]
#[allow(private_bounds)]
pub trait EditSubscriptionRecipients: RecurrentPayments
where
    <<Self as Gateway>::Payment as PaymentMarker>::PaymentMethod: InternalPaymentMethod,
{
    /// Edit the payment recipients (split configuration) of an existing subscription.
    ///
    /// # Parameters
    ///
    /// * `subscription_id` - ID of the subscription to edit
    /// * `recipients` - New payment recipients.
    ///   Implementations should validate that `recipients.validate_count(Self::MAX_ADDITIONAL_RECIPIENTS)`
    ///   returns Ok before processing.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success
    async fn edit_subscription_recipients(
        &self,
        subscription_id: SubscriptionId,
        recipients: Recipients,
    ) -> Result<(), Error>;
}

/// Optional trait for gateways that support changing subscription billing interval.
///
/// Less commonly supported than amount edits. Check gateway documentation for availability.
///
/// # Examples of Supporting Gateways
///
/// * Check specific gateway documentation (rare capability)
#[async_trait]
#[allow(private_bounds)]
pub trait EditSubscriptionInterval: RecurrentPayments
where
    <<Self as Gateway>::Payment as PaymentMarker>::PaymentMethod: InternalPaymentMethod,
{
    /// Edit the billing interval of an existing subscription.
    ///
    /// # Parameters
    ///
    /// * `subscription_id` - ID of the subscription to edit
    /// * `interval` - New billing interval
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` on success
    async fn edit_subscription_interval(
        &self,
        subscription_id: SubscriptionId,
        interval: SubscriptionInterval,
    ) -> Result<(), Error>;
}
