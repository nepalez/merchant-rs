use crate::types::{Money, SubscriptionInterval, SubscriptionStatus};

/// Information to build a subscription in Gateway adapters implementations.
pub struct Subscription<'a> {
    /// The unique subscription ID returned by the payment gateway.
    pub subscription_id: &'a str,
    /// The canonical status of the subscription.
    pub status: SubscriptionStatus,
    /// Billing interval (how often the customer is charged).
    pub interval: SubscriptionInterval,
    /// Amount charged per billing cycle.
    pub amount: Money,
    /// Subscription creation timestamp (Unix timestamp).
    pub created_at: i64,
    /// Next scheduled billing date (Unix timestamp, None if subscription is canceled/expired).
    pub next_billing_date: Option<i64>,
}
