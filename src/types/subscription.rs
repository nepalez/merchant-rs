use std::convert::TryFrom;

use crate::Error;
use crate::inputs::Subscription as Input;
use crate::types::{Money, SubscriptionId, SubscriptionInterval, SubscriptionStatus};

/// Subscription result returned by recurring payment operations
///
/// Represents the state of a recurring billing subscription.
/// Contains the subscription ID, current status, billing interval,
/// amount charged per cycle, and billing schedule information.
///
/// # Timestamps
///
/// All timestamps are Unix timestamps (seconds since epoch).
#[derive(Debug, Clone)]
pub struct Subscription {
    /// Unique subscription identifier from the payment gateway
    pub subscription_id: SubscriptionId,

    /// Current status of the subscription
    pub status: SubscriptionStatus,

    /// Billing interval (how often the customer is charged)
    pub interval: SubscriptionInterval,

    /// Amount charged per billing cycle
    pub amount: Money,

    /// Subscription creation timestamp (Unix timestamp)
    pub created_at: i64,

    /// Next scheduled billing date (Unix timestamp, None if subscription is canceled/expired)
    pub next_billing_date: Option<i64>,
}

impl<'a> TryFrom<Input<'a>> for Subscription {
    type Error = Error;

    fn try_from(input: Input<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            subscription_id: input.subscription_id.try_into()?,
            status: input.status,
            interval: input.interval,
            amount: input.amount,
            created_at: input.created_at,
            next_billing_date: input.next_billing_date,
        })
    }
}
