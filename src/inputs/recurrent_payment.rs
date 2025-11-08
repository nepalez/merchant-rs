use iso_currency::Currency;

use crate::inputs::Destinations;
use crate::types::SubscriptionInterval;

/// Insecure structure representing a recurrent payment.
pub struct RecurrentPayment<'a, Method: 'a> {
    /// The method of the payment to charge funds from
    pub method: Method,
    /// The currency for this payment
    pub currency: Currency,
    /// The payment destinations per billing cycle (platform or split between recipients)
    pub destinations: Destinations,
    /// The billing interval (how often the customer is charged)
    pub interval: SubscriptionInterval,
    /// The idempotency key
    pub idempotence_key: &'a str,
}
