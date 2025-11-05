use std::convert::TryFrom;

use crate::Error;
use crate::inputs::RecurrentPayment as Input;
use crate::internal::Validated;
use crate::types::{Money, PaymentMethod, SubscriptionInterval, TransactionIdempotenceKey};

/// Recurrent payment data for creating subscriptions.
///
/// Contains the payment method (e.g., CreditCard, StoredCard) along with subscription metadata
/// such as amount, billing interval, and idempotence key.
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
    /// The method of the payment to charge funds from
    pub method: Method,
    /// The amount to charge per billing cycle
    pub amount: Money,
    /// The billing interval (how often the customer is charged)
    pub interval: SubscriptionInterval,
    /// The idempotence key that can be used to prevent duplicate subscription creation
    pub idempotence_key: TransactionIdempotenceKey,
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
            amount: input.amount,
            interval: input.interval,
            idempotence_key: input.idempotence_key.try_into()?,
        }
        .validate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AsUnsafeRef;
    use crate::inputs;
    use crate::types::SubscriptionInterval;
    use crate::types::payment_methods::CreditCard;
    use iso_currency::Currency;
    use rust_decimal_macros::dec;

    fn valid_input() -> Input<'static, inputs::CreditCard<'static>> {
        Input {
            method: inputs::CreditCard {
                cvv: " 123 \n\t",
                number: " 4532-0151-1283-0366 \n\t",
                card_expiry: inputs::CardExpiry {
                    month: 12,
                    year: 2030,
                },
                holder_name: " john doe \n\t",
            },
            amount: Money {
                amount: dec!(100.00),
                currency: Currency::USD,
            },
            interval: SubscriptionInterval::Month(1),
            idempotence_key: " subscription-key-123 \n\t",
        }
    }

    #[test]
    fn constructed_from_valid_input() {
        let input = valid_input();
        let payment = RecurrentPayment::<CreditCard>::try_from(input).unwrap();

        unsafe {
            assert_eq!(payment.method.cvv.as_ref(), "123");
            assert_eq!(payment.method.number.as_ref(), "4532015112830366");
            assert_eq!(payment.method.card_expiry.month(), 12);
            assert_eq!(payment.method.card_expiry.year(), 2030);
            assert_eq!(payment.method.holder_name.as_ref(), "JOHN DOE");
            assert_eq!(payment.amount.amount, dec!(100.00));
            assert_eq!(payment.amount.currency, Currency::USD);
            assert_eq!(payment.interval, SubscriptionInterval::Month(1));
            assert_eq!(payment.idempotence_key.as_ref(), "subscription-key-123");
        }
    }

    #[test]
    fn rejects_invalid_payment_method() {
        let mut input = valid_input();
        input.method.cvv = "12";

        let result = RecurrentPayment::<CreditCard>::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_invalid_idempotence_key() {
        let mut input = valid_input();
        input.idempotence_key = "";

        let result = RecurrentPayment::<CreditCard>::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_zero_interval() {
        for interval in [SubscriptionInterval::Day(0), SubscriptionInterval::Month(0)] {
            let mut input = valid_input();
            input.interval = interval;

            let result = RecurrentPayment::<CreditCard>::try_from(input);
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }
    }
}
