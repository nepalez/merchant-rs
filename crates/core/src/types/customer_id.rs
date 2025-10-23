use std::fmt;
use std::str::FromStr;
use zeroize_derive::ZeroizeOnDrop;

use crate::error::Error;
use crate::internal::{Masked, PersonalData, sanitized::*, validated::*};

/// User identifier from an external vault or payment system
///
/// # Sanitization
/// * trims whitespaces,
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * length: 1-255 characters
///
/// # Data Protection
/// User IDs enable transaction correlation and user profiling,
/// and are considered PII (Personal Identifiable Information).
///
/// As such, they are:
/// * masked in logs (via `Debug` implementation) to display
///   the first 4 and the last 4 characters but not leaving less than 8 characters masked.
/// * exposed via the **unsafe** `as_str` method only,
///   forcing gateway developers to acknowledge the handling of sensitive data.
#[derive(Clone, ZeroizeOnDrop)]
pub struct CustomerId(String);

impl FromStr for CustomerId {
    type Err = Error;

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::sanitize(input).validated()
    }
}

impl fmt::Debug for CustomerId {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.masked_debug(f)
    }
}

impl PersonalData for CustomerId {
    unsafe fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for CustomerId {
    #[inline]
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        trim_whitespaces(&mut output.0, input);
        output
    }
}

impl Validated for CustomerId {
    #[inline]
    fn validate(&self) -> Result<(), String> {
        validate_length(&self.0, 1, 255)
    }
}

// SAFETY: The trait is safely implemented because exposing the first 4 and last 4 characters
// (but not leaving less than 8 characters masked):
// 1. Neither causes out-of-bounds access to potentially INVALID (empty) data by itself,
// 2. Nor leaks the essential part of the sensitive VALID data.
unsafe impl Masked for CustomerId {
    const TYPE_WRAPPER: &'static str = "CustomerId";

    #[inline]
    fn first_chars(&self) -> String {
        let len = self.0.len().saturating_sub(8).saturating_div(2).min(4);
        self.0.get(0..len).unwrap_or_default().to_string()
    }

    #[inline]
    fn last_chars(&self) -> String {
        let len = self.0.len().saturating_sub(8).saturating_div(2).min(4);
        self.0.get(0..len).unwrap_or_default().to_string()
    }
}
