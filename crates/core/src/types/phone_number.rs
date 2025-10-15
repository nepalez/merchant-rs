use std::fmt;
use std::str::FromStr;
use zeroize_derive::ZeroizeOnDrop;

use crate::error::Error;
use crate::internal::{Masked, PersonalData, sanitized::*, validated::*};

/// Personal phone number
///
/// # Sanitization
/// * removes all non-digit characters
/// * adds the leading `+` sign
///
/// # Validation
/// * length: 6-16 characters (including the `+` sign)
///
/// # Data Protection
/// Phone numbers enable contact with their holders
/// and SMS-based authentication, making them sensitive PII
/// (Personal Identifiable Information).
///
/// As such, they are:
/// * masked in logs (via `Debug` implementation) to display
///   the `+` sign and the last 2 digits only.
/// * exposed via the **unsafe** `as_str` method only,
///   forcing gateway developers to acknowledge the handling of sensitive data.
#[derive(Clone, ZeroizeOnDrop)]
pub struct PhoneNumber(String);

impl FromStr for PhoneNumber {
    type Err = Error;

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::sanitize(input).validated()
    }
}

impl fmt::Debug for PhoneNumber {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as Masked>::masked_debug(self, f)
    }
}

impl PersonalData for PhoneNumber {
    #[inline]
    unsafe fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl<'a> Sanitized<'a> for PhoneNumber {
    type Input = &'a str;

    #[inline]
    fn sanitize(input: Self::Input) -> Self {
        let mut output = Self(String::with_capacity(input.len() + 1));
        output.0.push('+');
        for c in input.chars() {
            if c.is_ascii_digit() {
                output.0.push(c);
            }
        }
        output
    }
}

impl Validated for PhoneNumber {
    #[inline]
    fn validate(&self) -> Result<(), String> {
        validate_length(&self.0, 6, 16)
    }
}

// SAFETY: The trait is safely implemented because exposing the `+` sign
// along with the last 2 characters:
// 1. Neither causes out-of-bounds access to potentially INVALID (empty) data,
//    due to fallbacks to the empty strings,
// 2. Nor leaks the sensitive VALID data (which has at least 5 digits) in total
//    and does not expose its actual length (this could be done by the first digits
//    which aren't exposed anyway).
unsafe impl Masked for PhoneNumber {
    const TYPE_WRAPPER: &'static str = "PhoneNumber";

    #[inline]
    fn first_chars(&self) -> String {
        self.0.get(0..1).unwrap_or_default().to_string()
    }

    #[inline]
    fn last_chars(&self) -> String {
        self.0
            .get(self.0.len() - 2..)
            .unwrap_or_default()
            .to_string()
    }
}
