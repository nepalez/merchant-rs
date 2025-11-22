use iso_currency::Currency;
use rust_decimal::Decimal;

use crate::Error;
use crate::types::{PaymentMethod, TransactionIdempotenceKey};

/// Payment information.
#[derive(Clone, Debug)]
#[allow(private_bounds)]
pub struct Payment<P: PaymentMethod> {
    pub(crate) payment_method: P,
    pub(crate) currency: Currency,
    pub(crate) total_amount: Decimal,
    pub(crate) base_amount: Decimal,
    pub(crate) idempotence_key: TransactionIdempotenceKey,
}

#[allow(private_bounds)]
impl<P: PaymentMethod> Payment<P> {
    /// The payment method.
    #[inline]
    pub fn payment_method(&self) -> &P {
        &self.payment_method
    }

    /// The currency of the payment.
    #[inline]
    pub fn currency(&self) -> Currency {
        self.currency
    }

    /// The total payment amount.
    #[inline]
    pub fn total_amount(&self) -> Decimal {
        self.total_amount
    }

    /// The amount going to the platform.
    #[inline]
    pub fn base_amount(&self) -> Decimal {
        self.base_amount
    }

    /// The idempotency key.
    #[inline]
    pub fn idempotence_key(&self) -> &TransactionIdempotenceKey {
        &self.idempotence_key
    }
}

impl<'a, M, P> TryFrom<crate::Payment<'a, M>> for Payment<P>
where
    P: PaymentMethod + TryFrom<M, Error = Error>,
{
    type Error = Error;

    fn try_from(input: crate::Payment<'a, M>) -> Result<Self, Self::Error> {
        Ok(Self {
            payment_method: input.payment_method.try_into()?,
            currency: input.currency,
            total_amount: input.total_amount,
            base_amount: input.base_amount,
            idempotence_key: input.idempotence_key.try_into()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::inputs;
    use crate::types::CreditCard;

    fn valid_input() -> inputs::Payment<'static, inputs::CreditCard<'static>> {
        inputs::Payment {
            payment_method: inputs::CreditCard {
                cvv: " 123 \n\t",
                number: " 4532-0151-1283-0366 \n\t",
                card_expiry: inputs::CardExpiry {
                    month: 12,
                    year: 2030,
                },
                holder_name: " john doe \n\t",
            },
            currency: Currency::USD,
            total_amount: Decimal::new(10000, 2),
            base_amount: Decimal::new(9500, 2),
            idempotence_key: " payment-123 \n\t",
        }
    }

    #[test]
    fn constructed_from_valid_input() {
        let input = valid_input();
        let payment = Payment::<CreditCard>::try_from(input).unwrap();

        assert_eq!(payment.currency, Currency::USD);
        assert_eq!(payment.total_amount, Decimal::new(10000, 2));
        assert_eq!(payment.base_amount, Decimal::new(9500, 2));
    }

    #[test]
    fn rejects_invalid_payment_method() {
        let mut input = valid_input();
        input.payment_method.cvv = "12";

        let result = Payment::<CreditCard>::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_invalid_idempotence_key() {
        let mut input = valid_input();
        input.idempotence_key = "";

        let result = Payment::<CreditCard>::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }
}
