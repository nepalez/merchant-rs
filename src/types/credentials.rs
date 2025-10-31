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

impl<'a, InputMethod, TypeMethod> TryFrom<Input<'a, InputMethod>> for Credentials<TypeMethod>
where
    InputMethod: Sized,
    TypeMethod: Sized + TryFrom<InputMethod, Error = Error>,
{
    type Error = Error;

    fn try_from(input: Input<'a, InputMethod>) -> Result<Self, Self::Error> {
        Ok(match input {
            Input::Plain(input_method) => Self::Plain(input_method.try_into()?),
            Input::Tokenized(token) => Self::Tokenized(token.try_into()?),
        })
    }
}
