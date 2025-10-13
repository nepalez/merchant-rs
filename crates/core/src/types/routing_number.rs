use std::convert::TryFrom;
use std::fmt;

use crate::error::{Error, Result};
use crate::types::{SafeWrapper, Sanitized, SecretString, Validated};

/// List of allowed separators in routing number input strings.
const NUMBER_SEPARATORS: [char; 3] = [' ', '-', '_'];
/// The required length for a bank routing number (digits only), typically 9 for ABA.
const ROUTING_LENGTH: usize = 9;
/// Standard fixed mask for logs.
const FIXED_MASK: &str = "********";

/// Represents a bank routing number, securely stored and validated.
/// Validation is strict (fixed length, digits only).
///
/// # SAFETY
///
/// **Architectural Justification for SecretBox:**
/// While the Routing Number is NOT classified as Sensitive Authentication Data (SAD) by PCI DSS,
/// it is critical **Personally Identifiable Information (PII)** and financial access data,
/// as it identifies the financial institution for ACH/wire transfers.
/// To enforce Defense-in-Depth, ensure guaranteed log masking, and prevent accidental data leakage,
/// it is treated with the same rigor as other sensitive data.
///
/// * The memory is forcefully zeroed on drop (guaranteed by SecretBox).
/// * The value is masked in `Debug` for log safety.
/// * Cloning is allowed for request resilience, but the cloned value is immediately re-wrapped in a new `SecretBox`.
/// * The value can only be accessed via the **unsafe** `with_exposed_secret` method, which forces developers to
///   acknowledge the handling of sensitive financial PII.
#[derive(Clone)]
pub struct RoutingNumber(SecretString);

impl RoutingNumber {
    /// Exposes the underlying Routing Number as a string slice.
    ///
    /// This method is designed for use by external payment adapter crates ONLY.
    ///
    /// # SAFETY
    ///
    /// This method is marked `unsafe` because it exposes highly sensitive data to the closure.
    ///
    /// The caller **MUST** ensure:
    /// 1. The processing within the closure does not copy
    ///    or store the exposed data in unsecured memory.
    /// 2. The data is consumed immediately and its exposure lifetime
    ///    is strictly minimal (e.g., for transmission).
    /// 3. **Any structure or variable containing the exposed `&str` reference
    ///    MUST NOT escape the closure, and any intermediate structure
    ///    containing a copy of the raw data (for example, the request)
    ///    MUST itself guarantee zeroization upon drop.**
    #[inline]
    pub unsafe fn with_exposed_secret<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&str) -> T,
    {
        // Safety: the safety contract is passed to the caller.
        unsafe { self.0.with_exposed_secret(f) }
    }
}

impl TryFrom<String> for RoutingNumber {
    type Error = Error;

    #[inline]
    fn try_from(input: String) -> Result<Self> {
        Self::try_from_string(input)
    }
}

impl fmt::Debug for RoutingNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("RoutingNumber").field(&FIXED_MASK).finish()
    }
}

// Sealed traits implementations

impl Sanitized for RoutingNumber {
    fn sanitize(input: String) -> Result<String> {
        let mut output = String::with_capacity(input.len());

        for c in input.chars() {
            if c.is_ascii_digit() {
                output.push(c);
            } else if NUMBER_SEPARATORS.contains(&c) {
                continue;
            } else {
                return Err(Error::validation_failed(format!(
                    "Input contains invalid character '{c}'.\
                     Only digits, spaces, and hyphens are allowed.",
                )));
            }
        }

        Ok(output)
    }
}

impl Validated for RoutingNumber {
    const TYPE_NAME: &'static str = "Routing Number";
    const MIN_LENGTH: usize = 9;
    const MAX_LENGTH: usize = 9;
    const EXTRA_CHARS: Option<&'static str> = None;
}

impl SafeWrapper for RoutingNumber {
    type Inner = SecretString;

    fn wrap(inner: Self::Inner) -> Self {
        Self(inner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    impl FromStr for RoutingNumber {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self> {
            Self::try_from(s.to_owned())
        }
    }
}
