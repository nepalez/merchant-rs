use std::convert::TryFrom;

use crate::Error;
use crate::inputs::Credentials as Input;
use crate::types::Token;

/// Credentials that can be tokenized
#[derive(Debug, Clone)]
pub enum Credentials<Plain: Sized> {
    Plain(Plain),
    Tokenized(Token),
}

impl<'a, Source, Target> TryFrom<Input<'a, Source>> for Credentials<Target>
where
    Source: Sized,
    Target: Sized + TryFrom<Source, Error = Error>,
{
    type Error = Error;

    fn try_from(input: Input<'a, Source>) -> Result<Self, Self::Error> {
        Ok(match input {
            Input::Plain(plain) => Self::Plain(plain.try_into()?),
            Input::Tokenized(token) => Self::Tokenized(token.try_into()?),
        })
    }
}
