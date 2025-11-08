use std::convert::TryFrom;

use iso_currency::Currency;

use crate::Error;
use crate::inputs::RecurrentPayment as Input;
use crate::internal::Validated;
use crate::types::{Destinations, PaymentMethod, SubscriptionInterval, TransactionIdempotenceKey};

/// Recurrent payment data for creating subscriptions.
///
/// Contains the payment method (e.g., CreditCard, StoredCard) along with subscription metadata
/// such as destination, currency, billing interval, and idempotence key.
///
/// Used for creating recurring billing subscriptions where the customer is automatically
/// charged at regular intervals.
///
/// # Type Parameter
///
/// * `Method` - The payment method type constrained by PaymentMethod marker trait
#[derive(Debug, Clone)]
#[allow(private_bounds)]
pub struct RecurrentPayment<Method: PaymentMethod> {
    pub(crate) method: Method,
    pub(crate) currency: Currency,
    pub(crate) destinations: Destinations,
    pub(crate) interval: SubscriptionInterval,
    pub(crate) idempotence_key: TransactionIdempotenceKey,
}

#[allow(private_bounds)]
impl<Method: PaymentMethod> RecurrentPayment<Method> {
    /// The method of the payment to charge funds from
    pub fn method(&self) -> &Method {
        &self.method
    }

    /// The currency for this payment
    pub fn currency(&self) -> Currency {
        self.currency
    }

    /// The payment destinations per billing cycle (platform or split between recipients)
    pub fn destinations(&self) -> &Destinations {
        &self.destinations
    }

    /// The billing interval (how often the customer is charged)
    pub fn interval(&self) -> &SubscriptionInterval {
        &self.interval
    }

    /// The idempotence key that can be used to prevent duplicate subscription creation
    pub fn idempotence_key(&self) -> &TransactionIdempotenceKey {
        &self.idempotence_key
    }
}

impl<Method: PaymentMethod> Validated for RecurrentPayment<Method> {
    fn validate(self) -> Result<Self, Error> {
        if self.interval.is_zero() {
            Err(Error::InvalidInput(
                "Subscription interval must be positive".into(),
            ))
        } else {
            Ok(self)
        }
    }
}

impl<'a, InputMethod, Method> TryFrom<Input<'a, InputMethod>> for RecurrentPayment<Method>
where
    InputMethod: 'a,
    Method: PaymentMethod + TryFrom<InputMethod, Error = Error>,
{
    type Error = Error;

    fn try_from(input: Input<'a, InputMethod>) -> Result<Self, Self::Error> {
        Self {
            method: input.method.try_into()?,
            currency: input.currency,
            destinations: input.destinations.try_into()?,
            interval: input.interval,
            idempotence_key: input.idempotence_key.try_into()?,
        }
        .validate()
    }
}

// TODO: Update tests after inputs are updated
// #[cfg(test)]
// mod tests { ... }
