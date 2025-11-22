use iso_currency::Currency;
use rust_decimal::Decimal;

use crate::Error;
use crate::types::{PaymentMethod, Recipients, TransactionIdempotenceKey};

/// Payment information with amount distribution to recipients.
#[derive(Clone, Debug)]
#[allow(private_bounds)]
pub struct SplitPayment<P: PaymentMethod> {
    pub(crate) payment_method: P,
    pub(crate) currency: Currency,
    pub(crate) total_amount: Decimal,
    pub(crate) base_amount: Decimal,
    pub(crate) idempotence_key: TransactionIdempotenceKey,
    pub(crate) recipients: Option<Recipients>,
}

#[allow(private_bounds)]
impl<P: PaymentMethod> SplitPayment<P> {
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

    /// The payment recipients.
    #[inline]
    pub fn recipients(&self) -> Option<&Recipients> {
        self.recipients.as_ref()
    }
}

impl<'a, M, P> TryFrom<crate::Payment<'a, M>> for SplitPayment<P>
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
            recipients: None,
        })
    }
}

impl<'a, M, P> TryFrom<crate::SplitPayment<'a, M>> for SplitPayment<P>
where
    P: PaymentMethod + TryFrom<M, Error = Error>,
{
    type Error = Error;

    fn try_from(input: crate::SplitPayment<'a, M>) -> Result<Self, Self::Error> {
        Ok(Self {
            payment_method: input.payment_method.try_into()?,
            currency: input.currency,
            total_amount: input.total_amount,
            base_amount: input.base_amount,
            idempotence_key: input.idempotence_key.try_into()?,
            recipients: input.recipients.map(TryFrom::try_from).transpose()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    use std::collections::HashMap;

    use crate::inputs::{self, DistributedValue::*};
    use crate::types::CreditCard;

    fn valid_split_payment_input() -> inputs::SplitPayment<'static, inputs::CreditCard<'static>> {
        let mut recipients = HashMap::new();
        recipients.insert("merchant_a", Amount(dec!(50.00)));
        recipients.insert("merchant_b", Percent(dec!(10.0)));

        inputs::SplitPayment {
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
            total_amount: dec!(100.00),
            base_amount: dec!(40.00),
            idempotence_key: " payment-123 \n\t",
            recipients: Some(recipients),
        }
    }

    fn valid_payment_input() -> inputs::Payment<'static, inputs::CreditCard<'static>> {
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
            total_amount: dec!(100.00),
            base_amount: dec!(100.00),
            idempotence_key: " payment-123 \n\t",
        }
    }

    #[test]
    fn constructed_from_valid_split_payment_input() {
        let input = valid_split_payment_input();
        let payment = SplitPayment::<CreditCard>::try_from(input).unwrap();

        assert_eq!(payment.currency, Currency::USD);
        assert_eq!(payment.total_amount, dec!(100.00));
        assert_eq!(payment.base_amount, dec!(40.00));
        assert_eq!(payment.recipients.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn constructed_from_valid_payment_input() {
        let input = valid_payment_input();
        let payment = SplitPayment::<CreditCard>::try_from(input).unwrap();

        assert_eq!(payment.currency, Currency::USD);
        assert_eq!(payment.total_amount, dec!(100.00));
        assert_eq!(payment.base_amount, dec!(100.00));
        assert!(payment.recipients.is_none());
    }

    #[test]
    fn rejects_invalid_payment_method() {
        let mut input = valid_split_payment_input();
        input.payment_method.cvv = "12";

        let result = SplitPayment::<CreditCard>::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_invalid_idempotence_key() {
        let mut input = valid_split_payment_input();
        input.idempotence_key = "";

        let result = SplitPayment::<CreditCard>::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_invalid_recipients() {
        let mut input = valid_split_payment_input();
        let mut bad_recipients = HashMap::new();
        bad_recipients.insert("a", Amount(dec!(-10.00)));
        input.recipients = Some(bad_recipients);

        let result = SplitPayment::<CreditCard>::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }
}
