use std::fmt;
use std::str::FromStr;
use zeroize_derive::ZeroizeOnDrop;

use crate::error::Error;
use crate::internal::{Exposed, Validated, sanitized::*};
use crate::types::insecure;

/// Postal code used in addresses
///
/// # Sanitization
/// * trims whitespaces,
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * length: 3-10 characters,
/// * only alphanumeric characters, spaces and dashes are allowed
///
/// # Data Protection
/// Postal codes can identify specific geographic areas
/// and when combined with other data, enable person identification,
/// making them PII (Personal Identifiable Information).
///
/// As such, they are:
/// * masked in logs (via `Debug` implementation) to display
///   up to the first 2 characters but no more than 1/3 of the code length,
/// * not exposed publicly except for a part of a request or response
///   via **unsafe** method `with_exposed_secret`.
#[derive(Clone, ZeroizeOnDrop)]
pub struct PostalCode(String);

impl FromStr for PostalCode {
    type Err = Error;

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::sanitize(input).validate()
    }
}

impl fmt::Debug for PostalCode {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.masked_debug(f)
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for PostalCode {
    #[inline]
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        trim_whitespaces(&mut output.0, input);
        output
    }
}

impl Validated for PostalCode {
    // We don't care about zeroization of the temporary data, that is not PII.
    #[inline]
    fn validate(self) -> Result<Self, Error> {
        self._validate_length(&self.0, 3, 10)?;
        self._validate_alphanumeric(&self.0, "- ")?;
        Ok(self)
    }
}

// SAFETY: The trait is safely implemented because exposing
// up to the first 2 characters, but no more than 1/3 of the code length:
// 1. Neither causes out-of-bounds access to potentially INVALID (empty) data,
//    due to fallbacks to the empty strings,
// 2. Nor leaks the sensitive VALID data because the first part of the code
//    points out to a broad geographical area.
unsafe impl Exposed for PostalCode {
    type Output<'a> = insecure::PostalCode<'a>;
    const TYPE_WRAPPER: &'static str = "PostalCode";

    #[inline]
    fn expose(&self) -> Self::Output<'_> {
        self.0.as_str()
    }

    #[inline]
    fn first_chars(&self) -> String {
        let len = (self.0.len() / 3).min(2);
        self.0.get(0..len).unwrap_or_default().to_string()
    }
}
