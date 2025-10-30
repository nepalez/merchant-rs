use std::convert::TryFrom;
use std::fmt;
use zeroize_derive::ZeroizeOnDrop;

use crate::Error;
use crate::internal::{
    AsUnsafeRef, ExternalPaymentSource, InternalPaymentSource, Masked, PaymentSource, Validated,
};

/// Tokenized credential from a payment processor or vault
///
/// # Validation
/// * length: 16-4096 characters,
/// * no leading or trailing spaces are allowed
///
/// # Data Protection
/// Tokens provide full access to payment credentials and enable unauthorized transactions,
/// making them sensitive authentication data (SAD).
///
/// As such, they are:
/// * fully masked in logs (via `Debug` implementation) to prevent any leaks,
/// * not exposed publicly except for a part of a request or response
///   via **unsafe** method `with_exposed_secret`.
#[derive(Clone, ZeroizeOnDrop)]
pub struct Token(String);

// Marker implementations

impl PaymentSource for Token {}
impl InternalPaymentSource for Token {}
impl ExternalPaymentSource for Token {}

// Converters

impl<'a> TryFrom<&'a str> for Token {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self(input.to_string()).validate()
    }
}

impl AsUnsafeRef<str> for Token {
    #[inline]
    unsafe fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Debug for Token {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.masked_debug(f)
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Validated for Token {
    #[inline]
    fn validate(self) -> Result<Self, Error> {
        self._validate_length(&self.0, 16, 4096)?;

        if self.0.trim() == self.0 {
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
unsafe impl Masked for Token {
    const TYPE_WRAPPER: &'static str = "Token";
}
