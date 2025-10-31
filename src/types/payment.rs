use std::convert::TryFrom;

use crate::Error;
use crate::inputs::Payment as Input;
use crate::types::{MerchantInitiatedType, Money, PaymentMethod, TransactionIdempotenceKey};

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
    /// The method of the payment to charge funds from
    method: Method,
    /// The amount to charge
    amount: Money,
    /// The idempotency key
    idempotence_key: TransactionIdempotenceKey,
    /// The scope of the payment initiated by the merchant
    /// (use `None` if the payment was initiated by a customer).
    merchant_initiated_type: Option<MerchantInitiatedType>,
}

#[allow(private_bounds)]
impl<Method: PaymentMethod> Payment<Method> {
    /// The method of the payment to charge funds from
    pub fn method(&self) -> &Method {
        &self.method
    }

    /// The amount to charge
    pub fn amount(&self) -> Money {
        self.amount
    }

    /// The idempotency key
    pub fn idempotence_key(&self) -> &TransactionIdempotenceKey {
        &self.idempotence_key
    }

    /// The scope of the payment initiated by the merchant
    /// (use `None` if the payment was initiated by a customer).
    pub fn merchant_initiated_type(&self) -> Option<MerchantInitiatedType> {
        self.merchant_initiated_type
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
        })
    }
}
