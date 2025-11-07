use std::convert::TryFrom;

use crate::Error;
use crate::inputs::Payment as Input;
use crate::types::{
    MerchantInitiatedType, Money, PaymentMethod, StoredCredentialUsage, TransactionIdempotenceKey,
};

/// Payment data with a raw payment method for direct processing.
///
/// Contains the payment method (e.g., CreditCard, BankAccount) along with transaction metadata
/// such as amount, idempotence key, and merchant-initiated transaction type.
///
/// Used for first-time payments where the customer provides payment details directly,
/// as opposed to tokenized payments using stored credentials.
///
/// # Type Parameter
///
/// * `Method` - The payment method type constrained by PaymentMethod marker trait
#[derive(Debug, Clone)]
#[allow(private_bounds)]
pub struct Payment<Method: PaymentMethod> {
    pub(crate) method: Method,
    pub(crate) amount: Money,
    pub(crate) idempotence_key: TransactionIdempotenceKey,
    pub(crate) merchant_initiated_type: Option<MerchantInitiatedType>,
    pub(crate) stored_credential_usage: Option<StoredCredentialUsage>,
}

#[allow(private_bounds)]
impl<Method: PaymentMethod> Payment<Method> {
    /// The method of the payment to charge funds from
    pub fn method(&self) -> &Method {
        &self.method
    }

    /// The amount to charge
    pub fn amount(&self) -> &Money {
        &self.amount
    }

    /// The idempotence key that can be used to retrieve the transaction id,
    /// and prevent duplication.
    pub fn idempotence_key(&self) -> &TransactionIdempotenceKey {
        &self.idempotence_key
    }

    /// The scope of the payment initiated by the merchant
    /// (use `None` if the payment was initiated by a customer).
    pub fn merchant_initiated_type(&self) -> &Option<MerchantInitiatedType> {
        &self.merchant_initiated_type
    }

    /// Indicates whether this is the first or later use of stored credentials.
    /// Use `None` for one-time payments where credentials are not stored.
    /// Required for Credential-on-File (COF) compliance with card networks.
    pub fn stored_credential_usage(&self) -> &Option<StoredCredentialUsage> {
        &self.stored_credential_usage
    }
}

impl<'a, InputMethod, Method> TryFrom<Input<'a, InputMethod>> for Payment<Method>
where
    InputMethod: 'a,
    Method: PaymentMethod + TryFrom<InputMethod, Error = Error>,
{
    type Error = Error;

    fn try_from(input: Input<'a, InputMethod>) -> Result<Self, Self::Error> {
        Ok(Self {
            method: input.method.try_into()?,
            amount: input.amount,
            idempotence_key: input.idempotence_key.try_into()?,
            merchant_initiated_type: input.merchant_initiated_type,
            stored_credential_usage: input.stored_credential_usage,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AsUnsafeRef;
    use crate::inputs;
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
            idempotence_key: " payment-key-123 \n\t",
            merchant_initiated_type: Some(MerchantInitiatedType::Recurring),
            stored_credential_usage: None,
        }
    }

    #[test]
    fn constructed_from_valid_input() {
        let input = valid_input();
        let payment = Payment::<CreditCard>::try_from(input).unwrap();

        unsafe {
            assert_eq!(payment.method.cvv.as_ref(), "123");
            assert_eq!(payment.method.number.as_ref(), "4532015112830366");
            assert_eq!(payment.method.card_expiry.month(), 12);
            assert_eq!(payment.method.card_expiry.year(), 2030);
            assert_eq!(payment.method.holder_name.as_ref(), "JOHN DOE");
            assert_eq!(payment.amount.amount, dec!(100.00));
            assert_eq!(payment.amount.currency, Currency::USD);
            assert_eq!(payment.idempotence_key.as_ref(), "payment-key-123");
            assert_eq!(
                payment.merchant_initiated_type,
                Some(MerchantInitiatedType::Recurring)
            );
        }
    }

    #[test]
    fn rejects_invalid_cvv() {
        let mut input = valid_input();
        input.method.cvv = "12";

        let result = Payment::<CreditCard>::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_invalid_pan() {
        let mut input = valid_input();
        input.method.number = "1234567890123";

        let result = Payment::<CreditCard>::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_invalid_card_expiry() {
        let mut input = valid_input();
        input.method.card_expiry.month = 13;

        let result = Payment::<CreditCard>::try_from(input);
        assert!(matches!(result, Err(Error::InvalidInput(_))));
    }

    #[test]
    fn rejects_invalid_holder_name() {
        let mut input = valid_input();
        input.method.holder_name = "X";

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
