use std::convert::TryFrom;

use iso_currency::Currency;

use crate::Error;
use crate::inputs::Subscription as Input;
use crate::types::{Destinations, SubscriptionId, SubscriptionInterval, SubscriptionStatus};

/// Subscription result returned by recurring payment operations
///
/// Represents the state of a recurring billing subscription.
/// Contains the subscription ID, current status, billing interval,
/// currency, payment destinations per billing cycle, and billing schedule information.
///
/// # Timestamps
///
/// All timestamps are Unix timestamps (seconds since epoch).
#[derive(Debug, Clone)]
pub struct Subscription {
    pub(crate) subscription_id: SubscriptionId,
    pub(crate) status: SubscriptionStatus,
    pub(crate) interval: SubscriptionInterval,
    pub(crate) currency: Currency,
    pub(crate) destinations: Destinations,
    pub(crate) created_at: i64,
    pub(crate) next_billing_date: Option<i64>,
}

impl Subscription {
    /// Unique subscription identifier from the payment gateway
    pub fn subscription_id(&self) -> &SubscriptionId {
        &self.subscription_id
    }

    /// Current status of the subscription
    pub fn status(&self) -> &SubscriptionStatus {
        &self.status
    }

    /// Billing interval (how often the customer is charged)
    pub fn interval(&self) -> &SubscriptionInterval {
        &self.interval
    }

    /// Currency of the subscription billing
    pub fn currency(&self) -> Currency {
        self.currency
    }

    /// Payment destinations per billing cycle (platform or split between recipients)
    pub fn destinations(&self) -> &Destinations {
        &self.destinations
    }

    /// Subscription creation timestamp (Unix timestamp)
    pub fn created_at(&self) -> &i64 {
        &self.created_at
    }

    /// Next scheduled billing date (Unix timestamp, None if subscription is canceled/expired)
    pub fn next_billing_date(&self) -> &Option<i64> {
        &self.next_billing_date
    }
}

impl<'a> TryFrom<Input<'a>> for Subscription {
    type Error = Error;

    fn try_from(input: Input<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            subscription_id: input.subscription_id.try_into()?,
            status: input.status,
            interval: input.interval,
            currency: input.currency,
            destinations: input.destinations.try_into()?,
            created_at: input.created_at,
            next_billing_date: input.next_billing_date,
        })
    }
}
