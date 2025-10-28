use std::convert::TryFrom;
use std::fmt;
use zeroize_derive::ZeroizeOnDrop;

use crate::Error;
use crate::internal::{Masked, Validated, sanitized::*};

/// Street address of a user
///
/// # Sanitization
/// * trims whitespaces,
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * length: 3-200 characters,
/// * only alphanumeric characters, spaces and dashes are allowed
///
/// # Data Protection
/// The street address precisely identifies a physical location of the user,
/// which makes it sensitive PII.
///
/// As such, they are:
/// * fully masked in logs (via `Debug` implementation) to prevent any leaks,
/// * not exposed publicly except for a part of a request or response
///   via **unsafe** method `with_exposed_secret`.
#[derive(Clone, ZeroizeOnDrop)]
pub struct StreetAddress(String);

impl<'a> TryFrom<&'a str> for StreetAddress {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self::sanitize(input).validate()
    }
}

impl fmt::Debug for StreetAddress {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.masked_debug(f)
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for StreetAddress {
    #[inline]
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        trim_whitespaces(&mut output.0, input);
        output
    }
}

impl Validated for StreetAddress {
    // We don't care about zeroization of the temporary data, that is not PII.
    #[inline]
    fn validate(self) -> Result<Self, Error> {
        self._validate_length(&self.0, 3, 200)?;
        self._validate_alphanumeric(&self.0, "- ")?;
        Ok(self)
    }
}

// SAFETY: The trait is safely implemented because it does not expose any data (full masking).
unsafe impl Masked for StreetAddress {
    const TYPE_WRAPPER: &'static str = "StreetAddress";
}
