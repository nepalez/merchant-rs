use std::convert::TryFrom;
use std::fmt;
use zeroize_derive::ZeroizeOnDrop;

use crate::Error;
use crate::internal::{AsUnsafeRef, Masked, Validated, sanitized::*};

/// Universal bank routing identifier
///
/// Used for ABA, Sort Code, BSB, IFSC, SWIFT/BIC, etc.
///
/// # Sanitization
/// * removes common separators: spaces, dashes, underscores and dots,
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * length: 6-11 characters,
/// * only alphanumeric characters are allowed
///
/// # Data Protection
/// While PCI DSS does NOT classify routing numbers as sensitive authentication data (SAD),
/// they identify specific bank branches and can be used
/// with account numbers for unauthorized transfers,
/// making them critical financial access data.
///
/// As such, they are:
/// * masked in logs (via `Debug` implementation) to display
///   the first and last characters (both in the upper case) only,
/// * not exposed publicly except for a part of a request or response
///   via **unsafe** method `with_exposed_secret`.
#[derive(Clone, ZeroizeOnDrop)]
pub struct RoutingNumber(String);

impl<'a> TryFrom<&'a str> for RoutingNumber {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let original = Self::sanitize(input).validate()?;
        Ok(Self(original.0.to_uppercase()))
    }
}

impl fmt::Debug for RoutingNumber {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.masked_debug(f)
    }
}

impl AsUnsafeRef<str> for RoutingNumber {
    #[inline]
    unsafe fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for RoutingNumber {
    #[inline]
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        filter_characters(&mut output.0, input, "-_.");
        output
    }
}

impl Validated for RoutingNumber {
    #[inline]
    fn validate(self) -> Result<Self, Error> {
        self._validate_length(&self.0, 6, 11)?;
        self._validate_alphanumeric(&self.0, "")?;
        Ok(self)
    }
}

// SAFETY: The trait is safely implemented because exposing the first 1 and last 1 character:
// 1. Neither causes out-of-bounds access to potentially INVALID (empty) data,
//    due to fallbacks to the empty strings,
// 2. Nor leaks the essential part of the sensitive VALID data which has at least 6 chars.
unsafe impl Masked for RoutingNumber {
    const TYPE_WRAPPER: &'static str = "RoutingNumber";

    #[inline]
    fn first_chars(&self) -> String {
        self.0.get(0..1).unwrap_or_default().to_uppercase()
    }

    #[inline]
    fn last_chars(&self) -> String {
        let len = self.0.len();
        self.0.get(len - 1..len).unwrap_or_default().to_uppercase()
    }
}
