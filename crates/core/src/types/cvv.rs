use std::fmt;
use std::str::FromStr;
use zeroize_derive::ZeroizeOnDrop;

use crate::error::Error;
use crate::internal::{HighlySecret, Masked, sanitized::*, validated::*};

/// Card Verification Value (CVV/CVC/CID)
///
/// # Sanitization
/// * trims whitespaces,
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * length: 3-4 characters,
/// * only digits are allowed
///
/// # Data Protection
/// PCI DSS classifies CVV as sensitive authentication data (SAD) that verifies
/// physical card possession and prevents card-not-present fraud.
///
/// As such, it is:
/// * fully masked in logs (via `Debug` implementation) to prevent any leaks,
/// * exposed via the **unsafe** `with_exposed_secret` method only,
///   forcing gateway developers to acknowledge the handling of sensitive data.
#[derive(Clone, ZeroizeOnDrop)]
#[allow(clippy::upper_case_acronyms)]
pub struct CVV(String);

impl FromStr for CVV {
    type Err = Error;

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::sanitize(input).validated()
    }
}

impl fmt::Debug for CVV {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as Masked>::masked_debug(self, f)
    }
}

impl<'a> HighlySecret<'a> for CVV {
    type Exposed = &'a str;

    #[inline]
    unsafe fn with_exposed_secret<T, F>(&'a self, f: F) -> T
    where
        F: FnOnce(Self::Exposed) -> T,
    {
        f(self.0.as_str())
    }
}

// --- Sealed traits (not parts of the public API) ---

impl<'a> Sanitized<'a> for CVV {
    type Input = &'a str;

    #[inline]
    fn sanitize(input: Self::Input) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        for c in input.trim().chars() {
            if !c.is_ascii_control() {
                output.0.push(c);
            }
        }
        output
    }
}

impl Validated for CVV {
    #[inline]
    fn validate(&self) -> Result<(), String> {
        validate_length(&self.0, 3, 4)?;
        validate_digits(&self.0, "")
    }
}

// SAFETY: The trait is safely implemented as it does NOT expose any part of CVV,
// fully protecting this sensitive authentication data in all contexts.
unsafe impl Masked for CVV {
    const TYPE_WRAPPER: &'static str = "CVV";
}
