use std::convert::TryFrom;

use crate::inputs::Payment as Input;
use crate::types::{payment_token::Source as PaymentSource, PaymentData, PaymentToken};
use crate::Error;

/// Information to create (either charge or authorize) a payment.
#[derive(Debug, Clone)]
#[allow(private_bounds)]
pub enum Payment<Source: PaymentSource> {
    Plain(PaymentData<Source>),
    Secure(PaymentToken<Source>),
}

impl<'a, InputSource, Source> TryFrom<Input<'a, InputSource>> for Payment<Source>
where
    InputSource: 'a,
    Source: PaymentSource + TryFrom<InputSource, Error = Error>,
{
    type Error = Error;

    fn try_from(input: Input<'a, InputSource>) -> Result<Self, Self::Error> {
        Ok(match input {
            Input::Secure(token) => Self::Secure(token.try_into()?),
            Input::Plain(source) => Self::Plain(source.try_into()?),
        })
    }
}
