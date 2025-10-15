use codes_iso_3166::part_1;
use std::fmt;
use std::str::FromStr;
use zeroize_derive::ZeroizeOnDrop;

use crate::error::Error;
use crate::internal::{Masked, PersonalData, sanitized::*, validated::*};

/// Street address of a user
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
/// Street addresses precisely identify physical locations
/// and enable user location tracking,
/// making them sensitive PII (Personal Identifiable Information).
///
/// As such, they are:
/// * fully masked in logs (via `Debug` implementation) to prevent any leaks,
/// * exposed via the **unsafe** `as_str` method only,
///   forcing gateway developers to acknowledge the handling of sensitive data.
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

impl PersonalData for StreetAddress {
    #[inline]
    unsafe fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl<'a> Sanitized<'a> for StreetAddress {
    type Input = &'a str;

    #[inline]
    fn sanitize(input: Self::Input) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        trim_whitespaces(&mut output.0, input);
        output
    }
}

impl Validated for StreetAddress {
    // We don't care about zeroization of the temporary data, that is not PII.
    #[inline]
    fn validate(&self) -> Result<(), String> {
        validate_length(&self.0, 3, 10)?;
        validate_alphanumeric(&self.0, "- ")
    }
}

// SAFETY: The trait is safely implemented because it does not expose any data (full masking).
unsafe impl Masked for StreetAddress {
    const TYPE_WRAPPER: &'static str = "StreetAddress";
}
