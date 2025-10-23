use std::fmt;
use std::str::FromStr;
use zeroize_derive::ZeroizeOnDrop;

use crate::error::Error;
use crate::internal::{Exposed, sanitized::*, validated::*};

/// External transaction identifier from a payment gateway
///
/// # Sanitization
/// * trims whitespaces,
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * length: 8-255 characters,
/// * only alphanumeric characters, dashes and underscores are allowed
///
/// # Data Protection
/// While neither PII nor classified as sensitive by PCI DSS, transaction identifiers
/// can be used to initiate operations (void, capture, refund) and access transaction details,
/// requiring access control at the highest level.
///
/// As such, they are:
/// * masked in logs (via `Debug` implementation) to display
///   the first and last characters (both in the upper case) only,
/// * not exposed publicly except for a part of a request or response
///   via **unsafe** method `with_exposed_secret`.
#[derive(Clone, ZeroizeOnDrop)]
pub struct TransactionId(String);

impl FromStr for TransactionId {
    type Err = Error;

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::sanitize(input).validated()
    }
}

impl fmt::Debug for TransactionId {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.masked_debug(f)
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for TransactionId {
    #[inline]
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        trim_whitespaces(&mut output.0, input);
        output
    }
}

impl Validated for TransactionId {
    #[inline]
    fn validate(&self) -> Result<(), String> {
        validate_length(&self.0, 8, 255)?;
        validate_alphanumeric(&self.0, "-_")
    }
}

// SAFETY: The trait is safely implemented because exposing the first 1 and last 1 character:
// 1. Neither causes out-of-bounds access to potentially INVALID (empty) data,
//    due to fallbacks to the empty strings,
// 2. Nor leaks the essential part of the sensitive VALID data which has at least 8 chars,
//    while also hiding the real length and case of the authorization ID.
unsafe impl Exposed for TransactionId {
    type Output<'a> = &'a str;

    #[inline]
    fn expose(&self) -> Self::Output<'_> {
        self.0.as_str()
    }

    const TYPE_WRAPPER: &'static str = "TransactionId";

    #[inline]
    fn first_chars(&self) -> String {
        self.0.get(0..1).unwrap_or_default().to_uppercase()
    }

    #[inline]
    fn last_chars(&self) -> String {
        self.0
            .get(self.0.len() - 1..)
            .unwrap_or_default()
            .to_uppercase()
    }
}
