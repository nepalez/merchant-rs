use std::convert::{AsRef, TryFrom};
use zeroize_derive::ZeroizeOnDrop;

use crate::Error;
use crate::internal::{Validated, sanitized::*};

/// Merchant's internal reference identifier for a transaction
///
/// # Sanitization
/// * trims whitespaces,
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * length: 1-255 characters
///
/// # Data Protection
/// This identifier is specifically designed for public usage and does not contain sensitive information.
///
/// Consequently, both `Debug` and `Display` are implemented without masking.
#[derive(Clone, Debug, ZeroizeOnDrop)]
pub struct TransactionIdempotenceKey(String);

impl<'a> TryFrom<&'a str> for TransactionIdempotenceKey {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self::sanitize(input).validate()
    }
}

impl AsRef<str> for TransactionIdempotenceKey {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for TransactionIdempotenceKey {
    #[inline]
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        trim_whitespaces(&mut output.0, input);
        output
    }
}

impl Validated for TransactionIdempotenceKey {
    #[inline]
    fn validate(self) -> Result<Self, Error> {
        self._validate_length(&self.0, 1, 255)?;
        Ok(self)
    }
}
