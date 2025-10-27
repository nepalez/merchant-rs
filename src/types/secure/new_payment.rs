use std::str::FromStr;

use crate::internal::{Exposed, Validated};
use crate::types::{
    Money,
    enums::MerchantInitiatedType,
    insecure,
    secure::{MerchantReferenceId, PaymentSource},
};
use crate::{Error, PaymentSourceType, PaymentSourceType::*};

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
    pub fn source_type(&self) -> PaymentSourceType {
        self.source.source_type()
    }
}

impl<'a> TryFrom<insecure::NewPayment<'a>> for Payment {
    type Error = Error;

    fn try_from(value: insecure::NewPayment<'a>) -> Result<Self, Self::Error> {
        Self {
            source: value.source.try_into()?,
            amount: value.amount,
            merchant_reference_id: FromStr::from_str(value.merchant_reference_id)?,
            merchant_initiated_type: value.merchant_initiated_type,
        }
        .validate()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Validated for Payment {
    fn validate(self) -> Result<Self, Error> {
        if self.merchant_initiated_type.is_some() {
            let source_type = self.source.source_type();
            match source_type {
                BankAccount | PaymentCard | SEPATransfer | TokenizedPayment => {}
                _ => {
                    return Err(Error::InvalidInput(format!(
                        "{source_type} does not support merchant initiated transactions."
                    )));
                }
            }
        }
        Ok(self)
    }
}

// SAFETY: The trait is safely implemented as:
// 1. it uses exposed versions of all inner types.
// 2. it uses `Debug` implementations of its values that mask sensitive data by themselves.
unsafe impl Exposed for Payment {
    type Output<'a> = insecure::NewPayment<'a>;
    const TYPE_WRAPPER: &'static str = "NewPayment";

    fn expose(&self) -> Self::Output<'_> {
        insecure::NewPayment {
            source: self.source.expose(),
            amount: self.amount,
            merchant_reference_id: self.merchant_reference_id.as_ref(),
            merchant_initiated_type: self.merchant_initiated_type,
        }
    }
}
