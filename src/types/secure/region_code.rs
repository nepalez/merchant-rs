use std::str::FromStr;
use zeroize_derive::ZeroizeOnDrop;

use crate::error::Error;
use crate::internal::{sanitized::*, validated::*};

/// Region code (state, province, etc.) in addresses
///
/// # Sanitization
/// * trims whitespaces,
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * length: 1-3 characters,
/// * case-insensitive (converted to uppercase)
///
/// # Data Protection
/// Region codes are NOT considered PII in any reasonable context,
/// as they represent broad geographic areas that cannot identify individuals.
///
/// Consequently, both `Debug` and `AsRef` are implemented without masking.
#[derive(Clone, Debug, ZeroizeOnDrop)]
pub struct RegionCode(String);

impl FromStr for RegionCode {
    type Err = Error;

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let original = Self::sanitize(input).validated()?;
        Ok(Self(original.0.to_uppercase()))
    }
}

impl AsRef<str> for RegionCode {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for RegionCode {
    #[inline]
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        trim_whitespaces(&mut output.0, input);
        output
    }
}

impl Validated for RegionCode {
    #[inline]
    fn validate(&self) -> Result<(), String> {
        validate_length(&self.0, 1, 3)
    }
}
