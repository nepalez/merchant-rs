use iso_currency::Currency;
use rust_decimal::Decimal;

use crate::inputs::{Recipients, SubscriptionInterval};

/// Insecure structure representing a recurrent payment.
pub struct RecurrentPayment<'a, Method: 'a> {
    /// The method of the payment to charge funds from
    pub method: Method,
    /// The currency for this payment
    pub currency: Currency,
    /// Total payment amount per billing cycle
    pub total_amount: Decimal,
    /// Optional payment recipients per billing cycle (None = platform receives all)
    pub recipients: Option<Recipients<'a>>,
    /// The billing interval (how often the customer is charged)
    pub interval: SubscriptionInterval,
    /// The idempotency key
    pub idempotence_key: &'a str,
}
