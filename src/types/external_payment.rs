use crate::Error;
use crate::inputs::ExternalPayment as Input;
use crate::types::{ExternalPaymentData, Transaction};

/// The data for completing an external payment along with the transaction.
pub struct ExternalPayment {
    pub transaction: Transaction,
    pub payment_data: ExternalPaymentData,
}

impl ExternalPayment {
    /// The transaction to complete.
    #[inline]
    pub fn transaction(&self) -> &Transaction {
        &self.transaction
    }

    /// The data for payment completion.
    #[inline]
    pub fn payment_data(&self) -> &ExternalPaymentData {
        &self.payment_data
    }
}

impl<'a> TryFrom<Input<'a>> for ExternalPayment {
    type Error = Error;

    fn try_from(input: Input<'a>) -> Result<Self, Self::Error> {
        Ok(Self {
            transaction: input.transaction.try_into()?,
            payment_data: input.payment_data.try_into()?,
        })
    }
}
