use std::convert::TryFrom;
use std::fmt;
use zeroize_derive::ZeroizeOnDrop;

use crate::Error;
use crate::internal::{AsUnsafeRef, Masked, Validated, sanitized::*};

/// Primary account number (PAN) from a payment card
///
/// # Sanitization
/// * removes dashes, underscores and spaces (common separators),
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * length: 13-19 characters,
/// * only digits are allowed,
/// * cannot start with 0,
/// * must pass the Luhn check (Mod 10)
///
/// # Data Protection
/// PCI DSS classifies PAN as sensitive authentication data (SAD) that provides
/// full access to cardholder funds and enables fraudulent transactions.
///
/// As such, it is:
/// * fully masked in logs (via `Debug` implementation) to prevent any leaks,
/// * not exposed publicly except for a part of a request or response
///   via **unsafe** method `with_exposed_secret`.
#[derive(Clone, ZeroizeOnDrop)]
pub struct PrimaryAccountNumber(String);

impl<'a> TryFrom<&'a str> for PrimaryAccountNumber {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self::sanitize(input).validate()
    }
}

impl fmt::Debug for PrimaryAccountNumber {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as Masked>::masked_debug(self, f)
    }
}

impl AsUnsafeRef<str> for PrimaryAccountNumber {
    #[inline]
    unsafe fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for PrimaryAccountNumber {
    #[inline]
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        filter_characters(&mut output.0, input, "-_");
        output
    }
}

impl Validated for PrimaryAccountNumber {
    fn validate(self) -> Result<Self, Error> {
        self._validate_length(&self.0, 13, 19)?;
        self._validate_alphanumeric(&self.0, "")?;

        if self.0.starts_with('0') {
            Err(Error::InvalidInput(format!("{self:?} cannot start with 0")))
        } else if !luhn3::valid(self.0.as_bytes()) {
            Err(Error::InvalidInput(format!(
                "{self:?} failed the Luhn check"
            )))
        } else {
            Ok(self)
        }
    }
}

// SAFETY: The trait is safely implemented because exposing the last 4 characters:
// 1. Neither causes out-of-bounds access to potentially INVALID (empty) data,
//    due to fallbacks to the empty strings,
// 2. Nor leaks the essential part of the sensitive VALID data which has at least 13 chars
//    (and this is explicitly enabled by the PCI DSS requirements).
unsafe impl Masked for PrimaryAccountNumber {
    const TYPE_WRAPPER: &'static str = "PrimaryAccountNumber";

    #[inline]
    fn last_chars(&self) -> String {
        let len = self.0.len();
        self.0.get(len - 4..len).unwrap_or_default().to_uppercase()
    }
}
