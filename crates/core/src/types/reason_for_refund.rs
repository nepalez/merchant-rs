use crate::error::Error;
use crate::internal::{Masked, PersonalData, sanitized::*, validated::*};
use std::fmt;
use std::str::FromStr;
use zeroize_derive::ZeroizeOnDrop;

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
/// * exposed via the **unsafe** `as_str` method only,
///   forcing gateway developers to acknowledge the handling of sensitive data.
#[derive(Clone, ZeroizeOnDrop)]
pub struct ReasonForRefund(String);

impl FromStr for ReasonForRefund {
    type Err = Error;

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self(input.to_string()).validated()
    }
}

impl fmt::Debug for ReasonForRefund {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.masked_debug(f)
    }
}

impl PersonalData for ReasonForRefund {
    unsafe fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl<'a> Sanitized<'a> for ReasonForRefund {
    type Input = &'a str;

    fn sanitize(input: Self::Input) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        trim_whitespaces(&mut output.0, input);
        output
    }
}

impl Validated for ReasonForRefund {
    fn validate(&self) -> std::result::Result<(), String> {
        validate_length(&self.0, 1, 255)
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
