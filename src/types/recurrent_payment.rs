use std::convert::TryFrom;

use iso_currency::Currency;
use rust_decimal::Decimal;

use crate::Error;
use crate::inputs::RecurrentPayment as Input;
use crate::types::{PaymentMethod, Recipients, SubscriptionInterval, TransactionIdempotenceKey};

/// Recurrent payment data for creating subscriptions.
///
/// Contains the payment method (e.g., CreditCard, StoredCard) along with subscription metadata
/// such as total amount, optional recipients (split), currency, billing interval, and idempotence key.
///
/// Used for creating recurring billing subscriptions where the customer is automatically
/// charged at regular intervals.
///
/// # Type Parameters
///
/// * `Method` - The payment method type constrained by PaymentMethod marker trait
#[derive(Debug, Clone)]
#[allow(private_bounds)]
pub struct RecurrentPayment<Method: PaymentMethod> {
    pub(crate) method: Method,
    pub(crate) currency: Currency,
    pub(crate) total_amount: Decimal,
    pub(crate) recipients: Option<Recipients>,
    pub(crate) interval: SubscriptionInterval,
    pub(crate) idempotence_key: TransactionIdempotenceKey,
}

#[allow(private_bounds)]
impl<Method: PaymentMethod> RecurrentPayment<Method> {
    /// The method of the payment to charge funds from
    #[inline]
    pub fn method(&self) -> &Method {
        &self.method
    }

    /// The currency for this payment
    #[inline]
    pub fn currency(&self) -> Currency {
        self.currency
    }

    /// Total payment amount per billing cycle
    #[inline]
    pub fn total_amount(&self) -> Decimal {
        self.total_amount
    }

    /// Optional payment recipients per billing cycle (None = platform receives all)
    #[inline]
    pub fn recipients(&self) -> Option<&Recipients> {
        self.recipients.as_ref()
    }

    /// The billing interval (how often the customer is charged)
    #[inline]
    pub fn interval(&self) -> &SubscriptionInterval {
        &self.interval
    }

    /// The idempotence key that can be used to prevent duplicate subscription creation
    #[inline]
    pub fn idempotence_key(&self) -> &TransactionIdempotenceKey {
        &self.idempotence_key
    }
}

impl<'a, InputMethod, Method> TryFrom<Input<'a, InputMethod>> for RecurrentPayment<Method>
where
    InputMethod: 'a,
    Method: PaymentMethod + TryFrom<InputMethod, Error = Error>,
{
    type Error = Error;

    fn try_from(input: Input<'a, InputMethod>) -> Result<Self, Self::Error> {
        Ok(Self {
            method: input.method.try_into()?,
            currency: input.currency,
            total_amount: input.total_amount,
            recipients: input.recipients.map(TryFrom::try_from).transpose()?,
            interval: input.interval.try_into()?,
            idempotence_key: input.idempotence_key.try_into()?,
        })
    }
}
