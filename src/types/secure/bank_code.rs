use std::str::FromStr;
use zeroize_derive::ZeroizeOnDrop;

use crate::error::Error;
use crate::internal::{sanitized::*, validated::*};

/// Code identifying the bank
///
/// # Sanitization
/// * removes characters used in financial systems for formatting: spaces, dashes, dots, slashes,
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * length: 2-34 characters,
/// * only alphanumeric characters are allowed
///
/// The adapter implementation can apply stricter validation rules later.
///
/// # Data Protection
/// This is a public value, neither secret nor even PII.
///
/// Consequently, both `Debug` and `AsRef` are implemented without masking.
#[derive(Clone, Debug, ZeroizeOnDrop)]
pub struct BankCode(String);

impl FromStr for BankCode {
    type Err = Error;

    #[inline]
    fn from_str(input: &str) -> Result<Self, Error> {
        Self::sanitize(input).validated()
    }
}

impl AsRef<str> for BankCode {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for BankCode {
    #[inline]
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        filter_characters(&mut output.0, input, ".-/");
        output
    }
}

impl Validated for BankCode {
    #[inline]
    fn validate(&self) -> Result<(), String> {
        validate_length(&self.0, 2, 34)?;
        validate_alphanumeric(&self.0, "")
    }
}
