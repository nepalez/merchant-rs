use std::convert::TryFrom;

use crate::Error;
use crate::inputs::PaymentData as Input;
use crate::internal::PaymentSource;
use crate::types::{MerchantInitiatedType, Money, TransactionIdempotenceKey};

/// Payment data with a raw payment source for direct processing.
///
/// Contains the payment source (e.g., CreditCard, BankAccount) along with transaction metadata
/// such as amount, idempotence key, and merchant-initiated transaction type.
///
/// Used for first-time payments where the customer provides payment details directly,
/// as opposed to tokenized payments using stored credentials.
///
/// # Type Parameter
///
/// * `Source` - The payment source type constrained by PaymentSource marker trait
#[derive(Debug, Clone)]
#[allow(private_bounds)]
pub struct PaymentData<Source: PaymentSource> {
    /// The source of the payment to charge funds from
    source: Source,
    /// The amount to charge
    amount: Money,
    /// The idempotency key
    idempotence_key: TransactionIdempotenceKey,
    /// The scope of the payment initiated by the merchant
    /// (use `None` if the payment was initiated by a customer).
    merchant_initiated_type: Option<MerchantInitiatedType>,
}

#[allow(private_bounds)]
impl<Source: PaymentSource> PaymentData<Source> {
    /// The source of the payment to charge funds from
    pub fn source(&self) -> &Source {
        &self.source
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

impl<'a, InputSource, Source> TryFrom<Input<'a, InputSource>> for PaymentData<Source>
where
    InputSource: 'a,
    Source: PaymentSource + TryFrom<InputSource, Error = Error>,
{
    type Error = Error;

    fn try_from(input: Input<'a, InputSource>) -> Result<Self, Self::Error> {
        Ok(Self {
            source: input.source.try_into()?,
            amount: input.amount,
            idempotence_key: input.idempotence_key.try_into()?,
            merchant_initiated_type: input.merchant_initiated_type,
        })
    }
}
