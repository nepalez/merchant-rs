use std::convert::TryFrom;

use iso_currency::Currency;

use crate::inputs::Subscription as Input;
use crate::types::{Recipients, SubscriptionId, SubscriptionInterval};
use crate::{Error, SubscriptionStatus};

/// Subscription result returned by recurring payment operations
///
/// Represents the state of a recurring billing subscription.
/// Contains the subscription ID, current status, billing interval,
/// currency, payment recipients per billing cycle, and billing schedule information.
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
    pub(crate) recipients: Option<Recipients>,
    pub(crate) created_at: i64,
    pub(crate) next_billing_date: Option<i64>,
}

impl Subscription {
    /// Unique subscription identifier from the payment gateway
    #[inline]
    pub fn subscription_id(&self) -> &SubscriptionId {
        &self.subscription_id
    }

    /// Current status of the subscription
    #[inline]
    pub fn status(&self) -> &SubscriptionStatus {
        &self.status
    }

    /// Billing interval (how often the customer is charged)
    #[inline]
    pub fn interval(&self) -> &SubscriptionInterval {
        &self.interval
    }

    /// Currency of the subscription billing
    #[inline]
    pub fn currency(&self) -> Currency {
        self.currency
    }

    /// Payment recipients per billing cycle (None = platform receives all)
    #[inline]
    pub fn recipients(&self) -> Option<&Recipients> {
        self.recipients.as_ref()
    }

    /// Subscription creation timestamp (Unix timestamp)
    #[inline]
    pub fn created_at(&self) -> &i64 {
        &self.created_at
    }

    /// Next scheduled billing date (Unix timestamp, None if subscription is canceled/expired)
    #[inline]
    pub fn next_billing_date(&self) -> Option<i64> {
        self.next_billing_date
    }
}

impl<'a> TryFrom<Input<'a>> for Subscription {
    type Error = Error;

    fn try_from(input: Input<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            subscription_id: input.subscription_id.try_into()?,
            status: input.status,
            interval: input.interval.try_into()?,
            currency: input.currency,
            recipients: input.recipients.map(TryFrom::try_from).transpose()?,
            created_at: input.created_at,
            next_billing_date: input.next_billing_date,
        })
    }
}
