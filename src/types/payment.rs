use std::convert::TryFrom;

use crate::Error;
use crate::inputs::Payment as Input;
use crate::internal::PaymentSource;
use crate::types::{PaymentData, PaymentToken};

/// Payment information for initiating a transaction.
///
/// Represents payment data in one of two forms:
/// * `Plain` - Direct payment with raw payment source data
/// * `Secure` - Tokenized payment using a previously stored token
///
/// # Usage
///
/// Use `Plain` for first-time payments where the customer provides payment details directly.
/// Use `Secure` for recurring payments or when using tokens from vault/tokenization services.
///
/// # Type Parameter
///
/// * `Source` - The payment source type (CreditCard, BankAccount, etc.) constrained by marker trait
#[derive(Debug, Clone)]
#[allow(private_bounds)]
pub enum Payment<Source: PaymentSource> {
    /// Direct payment with raw payment source data
    Plain(PaymentData<Source>),
    /// Tokenized payment using stored token
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
