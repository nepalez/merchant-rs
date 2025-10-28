use std::fmt;
use zeroize_derive::ZeroizeOnDrop;

use crate::Error;
use crate::internal::{Masked, Validated, sanitized::*};

/// Optional administrative text explaining the reason for a refund
///
/// # Sanitization
/// * trims leading and trailing whitespace
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * length: 1-255 characters
///
/// # Data Protection
/// Free-text fields may contain arbitrary PII (names, emails, phone numbers)
/// if merchants are poorly trained. Showing length only prevents accidental PII
/// exposure while maintaining the debugging utility.
///
/// As such, it is:
/// * masked in logs (via `Debug` implementation) to display only the length of the content,
/// * not exposed publicly except for a part of a request or response
///   via **unsafe** method `with_exposed_secret`.
#[derive(Clone, ZeroizeOnDrop)]
pub struct ReasonForRefund(String);

impl<'a> TryFrom<&'a str> for ReasonForRefund {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self(input.to_string()).validate()
    }
}

impl fmt::Debug for ReasonForRefund {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.masked_debug(f)
    }
}

impl AsRef<str> for ReasonForRefund {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for ReasonForRefund {
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        trim_whitespaces(&mut output.0, input);
        output
    }
}

impl Validated for ReasonForRefund {
    fn validate(self) -> Result<Self, Error> {
        self._validate_length(&self.0, 1, 255)?;
        Ok(self)
    }
}

// SAFETY: The trait is safely implemented as it does NOT expose any part of the internal value.
unsafe impl Masked for ReasonForRefund {
    const TYPE_WRAPPER: &'static str = "ReasonForRefund";

    fn masked_debug(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let masked = format!("[{} chars]", self.0.chars().count());
        f.debug_tuple(Self::TYPE_WRAPPER).field(&masked).finish()
    }
}
