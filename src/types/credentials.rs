use std::convert::TryFrom;

use crate::Error;
use crate::types::Token;

/// Credentials that can be tokenized
#[derive(Debug, Clone)]
pub enum Credentials<Plain: Sized> {
    Plain(Plain),
    Tokenized(Token),
}

impl<'a, InputMethod, TypeMethod> TryFrom<crate::Credentials<'a, InputMethod>>
    for Credentials<TypeMethod>
where
    InputMethod: Sized,
    TypeMethod: Sized + TryFrom<InputMethod, Error = Error>,
{
    type Error = Error;

    fn try_from(input: crate::Credentials<'a, InputMethod>) -> Result<Self, Self::Error> {
        Ok(match input {
            crate::Credentials::Plain(input_method) => Self::Plain(input_method.try_into()?),
            crate::Credentials::Tokenized(token) => Self::Tokenized(token.try_into()?),
        })
    }
}
