use crate::types::{Money, SubscriptionInterval};

/// Insecure structure representing a recurrent payment.
pub struct RecurrentPayment<'a, Method: 'a> {
    /// The method of the payment to charge funds from
    pub method: Method,
    /// The amount to charge per billing cycle
    pub amount: Money,
    /// The billing interval (how often the customer is charged)
    pub interval: SubscriptionInterval,
    /// The idempotency key
    pub idempotence_key: &'a str,
}
