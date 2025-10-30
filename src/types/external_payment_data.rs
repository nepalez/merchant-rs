use crate::Error;
use crate::inputs::ExternalPaymentData as Input;

/// The data for completing an external payment.
#[derive(Clone, Debug)]
pub struct ExternalPaymentData {}

impl<'a> TryFrom<Input<'a>> for ExternalPaymentData {
    type Error = Error;

    fn try_from(_input: Input<'a>) -> Result<Self, Self::Error> {
        Ok(Self {})
    }
}
