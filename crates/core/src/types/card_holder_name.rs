use std::fmt;
use std::str::FromStr;
use zeroize_derive::ZeroizeOnDrop;

use crate::error::Error;
use crate::internal::{Exposed, sanitized::*, validated::*};

/// Cardholder name as it appears on a payment card
///
/// # Sanitization
/// * trims whitespaces,
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * length: 3-26 characters (EMV and ISO/IEC 7813 standard),
/// * only ASCII alphabetic characters, spaces, dashes, apostrophes and dots are allowed,
/// * any non-Latin character (e.g., Cyrillic, Chinese) fails validation
///
/// # Data Protection
/// While PCI DSS does NOT classify cardholder names as sensitive authentication data (SAD),
/// they are critical PII and financial access data that can be associated with their owners.
///
/// As such, they are:
/// * masked in logs (via `Debug` implementation) to display
///   the first and last characters (both in the upper case) only,
///   which prevents leaking short names,
/// * not exposed publicly except for a part of a request or response
///   via **unsafe** method `with_exposed_secret`.
#[derive(Clone, ZeroizeOnDrop)]
pub struct CardHolderName(String);

impl FromStr for CardHolderName {
    type Err = Error;

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let original = Self::sanitize(input).validated()?;
        Ok(Self(original.0.to_uppercase()))
    }
}

impl fmt::Debug for CardHolderName {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as Exposed>::masked_debug(self, f)
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for CardHolderName {
    #[inline]
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        trim_whitespaces(&mut output.0, input);
        output
    }
}

impl Validated for CardHolderName {
    #[inline]
    fn validate(&self) -> Result<(), String> {
        validate_length(&self.0, 3, 26)?;
        validate_alphabetic(&self.0, " -'.")
    }
}

// SAFETY: The trait is safely implemented because exposing the first 1 and last 1 character:
// 1. Neither causes out-of-bounds access to potentially INVALID (empty) data,
//    due to fallbacks to the empty strings,
// 2. Nor leaks the essential part of the sensitive VALID data
//    due to hiding the real length of the name.
unsafe impl Exposed for CardHolderName {
    type Output<'a> = &'a str;

    const TYPE_WRAPPER: &'static str = "CardHolderName";

    #[inline]
    fn expose(&self) -> Self::Output<'_> {
        self.0.as_str()
    }

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
