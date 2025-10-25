use std::fmt;
use std::str::FromStr;
use zeroize_derive::ZeroizeOnDrop;

use crate::error::Error;
use crate::internal::{Exposed, sanitized::*, validated::*};
use crate::types::insecure;

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
/// * not exposed publicly except for a part of a request or response
///   via **unsafe** method `with_exposed_secret`.
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
        <Self as Exposed>::masked_debug(self, f)
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for CVV {
    #[inline]
    fn sanitize(input: &str) -> Self {
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
unsafe impl Exposed for CVV {
    type Output<'a> = insecure::CVV<'a>;
    const TYPE_WRAPPER: &'static str = "CVV";

    #[inline]
    fn expose(&self) -> Self::Output<'_> {
        self.0.as_str()
    }
}
