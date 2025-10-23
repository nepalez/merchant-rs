use std::fmt;
use std::str::FromStr;
use zeroize_derive::ZeroizeOnDrop;

use crate::Error;
use crate::internal::{HighlySecret, Masked, sanitized::*, validated::*};

/// National identification number of the user
///
/// # Sanitization
/// * removes common separators: spaces, dashes, dots, underscores, and apostrophes,
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * length: 7-18 characters,
/// * only alphanumeric characters are allowed
///
/// # Data Protection
/// National IDs can precisely identify individuals and enable identity theft or fraud,
/// making them highly sensitive PII (Personal Identifiable Information).
///
/// As such, they are:
/// * masked in logs (via `Debug` implementation) to display
///   the first and last characters only,
/// * exposed via the **unsafe** `with_exposed_secret` method only,
///   forcing gateway developers to acknowledge the handling of sensitive data.
#[derive(Clone, ZeroizeOnDrop)]
pub struct NationalId(String);

impl FromStr for NationalId {
    type Err = Error;

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::sanitize(input).validated()
    }
}

impl<'a> HighlySecret<'a> for NationalId {
    type Exposed = &'a str;

    #[inline]
    unsafe fn with_exposed_secret<T, F>(&'a self, f: F) -> T
    where
        F: FnOnce(Self::Exposed) -> T,
    {
        f(self.0.as_str())
    }
}

impl fmt::Debug for NationalId {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as Masked>::masked_debug(self, f)
    }
}

// --- Sealed traits (not parts of the public API) ---

impl<'a> Sanitized<'a> for NationalId {
    type Input = &'a str;

    #[inline]
    fn sanitize(input: Self::Input) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        filter_characters(&mut output.0, input, "'.-_");
        output
    }
}

impl Validated for NationalId {
    #[inline]
    fn validate(&self) -> Result<(), String> {
        validate_length(&self.0, 7, 18)?;
        validate_alphanumeric(&self.0, "")
    }
}

// SAFETY: The trait is safely implemented because exposing the first 1 and last 1 character:
// 1. Neither causes out-of-bounds access to potentially INVALID (empty) data,
//    due to fallbacks to the empty strings,
// 2. Nor leaks the essential part of the sensitive VALID data which has at least 7 chars.
unsafe impl Masked for NationalId {
    const TYPE_WRAPPER: &'static str = "NationalId";

    #[inline]
    fn first_chars(&self) -> String {
        self.0.get(0..1).unwrap_or_default().to_string()
    }

    #[inline]
    fn last_chars(&self) -> String {
        let len = self.0.len();
        self.0.get(len - 1..len).unwrap_or_default().to_string()
    }
}
