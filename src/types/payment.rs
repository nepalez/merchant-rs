use std::convert::TryFrom;

use crate::Error;
use crate::inputs::Payment as Input;
use crate::types::{MerchantInitiatedType, MerchantReferenceId, Money, PaymentSource};

/// Information to create (either charge or authorize) a payment.
#[derive(Debug, Clone)]
pub struct Payment {
    /// The source of the payment to charge funds from
    source: PaymentSource,
    /// The amount to charge
    amount: Money,
    /// The idempotency key
    merchant_reference_id: MerchantReferenceId,
    /// The scope of the payment initiated by the merchant
    /// (use `None` if the payment was initiated by a customer).
    merchant_initiated_type: Option<MerchantInitiatedType>,
}

impl Payment {
    /// The source of the payment to charge funds from
    pub fn source(&self) -> &PaymentSource {
        &self.source
    }

    /// The amount to charge
    pub fn amount(&self) -> Money {
        self.amount
    }

    /// The idempotency key
    pub fn merchant_reference_id(&self) -> &MerchantReferenceId {
        &self.merchant_reference_id
    }

    /// The scope of the payment initiated by the merchant
    /// (use `None` if the payment was initiated by a customer).
    pub fn merchant_initiated_type(&self) -> Option<MerchantInitiatedType> {
        self.merchant_initiated_type
    }
}

impl<'a> TryFrom<Input<'a>> for Payment {
    type Error = Error;

    fn try_from(input: Input<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            source: input.source.try_into()?,
            amount: input.amount,
            merchant_reference_id: input.merchant_reference_id.try_into()?,
            merchant_initiated_type: input.merchant_initiated_type,
        })
    }
}
