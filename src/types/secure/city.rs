use std::str::FromStr;
use zeroize_derive::ZeroizeOnDrop;

use crate::error::Error;
use crate::internal::{sanitized::*, validated::*};

/// City name in addresses
///
/// # Sanitization
/// * trims whitespaces,
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * length: 1-100 characters
///
/// # Data Protection
/// City names are NOT considered PII in any reasonable context,
/// as they represent broad geographic areas that cannot identify individuals.
///
/// Consequently, both `Debug` and `AsRef` are implemented without masking.
#[derive(Clone, Debug, ZeroizeOnDrop)]
pub struct City(String);

impl FromStr for City {
    type Err = Error;

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::sanitize(input).validated()
    }
}

impl AsRef<str> for City {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for City {
    #[inline]
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        trim_whitespaces(&mut output.0, input);
        output
    }
}

impl Validated for City {
    #[inline]
    fn validate(&self) -> Result<(), String> {
        validate_length(&self.0, 1, 100)
    }
}
