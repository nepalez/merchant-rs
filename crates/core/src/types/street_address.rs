use std::fmt;
use std::str::FromStr;
use zeroize_derive::ZeroizeOnDrop;

use crate::error::Error;
use crate::internal::{Exposed, sanitized::*, validated::*};

/// Street address of a user
///
/// # Sanitization
/// * trims whitespaces,
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * length: 3-200 characters,
/// * only alphanumeric characters, spaces and dashes are allowed
///
/// # Data Protection
/// Street addresses precisely identify physical locations
/// and enable user location tracking,
/// making them sensitive PII (Personal Identifiable Information).
///
/// As such, they are:
/// * fully masked in logs (via `Debug` implementation) to prevent any leaks,
/// * not exposed publicly except for a part of a request or response
///   via **unsafe** method `with_exposed_secret`.
#[derive(Clone, ZeroizeOnDrop)]
pub struct StreetAddress(String);

impl FromStr for StreetAddress {
    type Err = Error;

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::sanitize(input).validated()
    }
}

impl fmt::Debug for StreetAddress {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.masked_debug(f)
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for StreetAddress {
    #[inline]
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        trim_whitespaces(&mut output.0, input);
        output
    }
}

impl Validated for StreetAddress {
    // We don't care about zeroization of the temporary data, that is not PII.
    #[inline]
    fn validate(&self) -> Result<(), String> {
        validate_length(&self.0, 3, 200)?;
        validate_alphanumeric(&self.0, "- ")
    }
}

// SAFETY: The trait is safely implemented because it does not expose any data (full masking).
unsafe impl Exposed for StreetAddress {
    type Output<'a> = &'a str;
    const TYPE_WRAPPER: &'static str = "StreetAddress";

    #[inline]
    fn expose(&self) -> Self::Output<'_> {
        self.0.as_str()
    }
}
