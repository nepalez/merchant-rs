use std::str::FromStr;
use zeroize_derive::ZeroizeOnDrop;

use crate::error::Error;
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
pub struct MerchantReferenceId(String);

impl FromStr for MerchantReferenceId {
    type Err = Error;

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::sanitize(input).validate()
    }
}

impl AsRef<str> for MerchantReferenceId {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for MerchantReferenceId {
    #[inline]
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        trim_whitespaces(&mut output.0, input);
        output
    }
}

impl Validated for MerchantReferenceId {
    #[inline]
    fn validate(self) -> Result<Self, Error> {
        self._validate_length(&self.0, 1, 255)?;
        Ok(self)
    }
}
