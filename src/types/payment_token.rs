use std::any::type_name;
use std::fmt;
use std::fmt::Formatter;
use std::marker::PhantomData;
use zeroize_derive::ZeroizeOnDrop;

use crate::Error;
use crate::internal::{AsUnsafeRef, Masked, PaymentSource, Validated};
use crate::types::{BankAccount, CreditCard, InstantAccount, SEPAAccount, Token};

/// Marker trait for payment sources that support immediate payments
/// and 3D Secure authentication.
pub(crate) trait Source: PaymentSource {}
impl Source for BankAccount {}
impl Source for SEPAAccount {}
impl Source for InstantAccount {}
impl Source for CreditCard {}
impl Source for Token {}

#[derive(Clone, ZeroizeOnDrop)]
#[allow(private_bounds)]
pub struct PaymentToken<Content: Source> {
    value: String,
    _content: PhantomData<Content>,
}

impl<'a, Content: Source> TryFrom<&'a str> for PaymentToken<Content> {
    type Error = Error;

    fn try_from(token: &'a str) -> Result<Self, Self::Error> {
        Ok(Self {
            value: token.to_string(),
            _content: PhantomData,
        })
    }
}

impl<Content: Source> AsUnsafeRef<str> for PaymentToken<Content> {
    unsafe fn as_ref(&self) -> &str {
        self.value.as_str()
    }
}

impl<Content: Source> fmt::Debug for PaymentToken<Content> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.masked_debug(f)
    }
}

// --- Sealed traits (not parts of the public API) ---

impl<Content: Source> Validated for PaymentToken<Content> {
    #[inline]
    fn validate(self) -> Result<Self, Error> {
        self._validate_length(&self.value, 16, 4096)?;

        if self.value.trim() == self.value {
            Ok(self)
        } else {
            Err(Error::InvalidInput(format!(
                "{self:?} contains trailing whitespaces"
            )))
        }
    }
}

// SAFETY: The trait is safely implemented as it does NOT expose any part of the token,
// fully protecting this sensitive authentication data from exposure in debug output.
unsafe impl<Content: Source> Masked for PaymentToken<Content> {
    const TYPE_WRAPPER: &'static str = "PaymentToken";

    #[inline]
    fn masked_debug(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let wrapper = format!("{}<{}>", Self::TYPE_WRAPPER, type_name::<Content>());

        f.debug_tuple(&wrapper).field(&Self::MASKING_STR).finish()
    }
}
