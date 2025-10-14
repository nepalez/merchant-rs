use std::convert::TryFrom;
use std::fmt;

use crate::error::*;
use crate::internal::*;

/// Standard fixed mask for logs.
const FIXED_MASK: &str = "***";

/// The Card Verification Value (CVV/CVC/CID).
///
/// # SAFETY
///
/// * The memory is forcefully zeroed on drop.
/// * Cloning is allowed for request resilience, but the cloned value is immediately re-wrapped in a new `SecretBox`.
/// * The value is masked in `Debug`, `Display`, and `Serialize`.
/// * The value can only be accessed via the **unsafe** `expose_secret` method.
#[derive(Clone)]
#[allow(clippy::upper_case_acronyms)]
pub struct CVV(SecretString);

impl CVV {
    /// Exposes the underlying Primary Account Number (PAN) to a closure for temporary processing.
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
        // SAFETY: the safety contract is passed to the caller.
        unsafe { self.0.with_exposed_secret(f) }
    }
}

// SAFETY:
//
// The trait is safely implemented because:
// 1. SecretString is used as the inner type which ensures memory zeroization on drop.
// 2. No characters are exposed in Debug implementation, only a fixed mask is shown.
unsafe impl SafeWrapper for CVV {
    type Inner = SecretString;

    #[inline]
    fn wrap(inner: Self::Inner) -> Self {
        Self(inner)
    }

    #[inline]
    unsafe fn inner(&self) -> &Self::Inner {
        &self.0
    }
}

impl Sanitized for CVV {}

impl Validated for CVV {
    const TYPE_NAME: &'static str = "CVV";
    const MIN_LENGTH: usize = 3;
    const MAX_LENGTH: usize = 4;
    // Use custom validation of strict ASCII digits only
    const EXTRA_CHARS: Option<&'static str> = None;

    #[inline]
    fn validate(input: &str) -> Result<()> {
        Self::validate_length(input)?;

        if !input.chars().all(|c| c.is_ascii_digit()) {
            return Err(Error::validation_failed(format!(
                "{} must contain digits (0-9) only",
                Self::TYPE_NAME,
            )));
        }

        Ok(())
    }
}

impl TryFrom<String> for CVV {
    type Error = Error;

    #[inline]
    fn try_from(input: String) -> Result<Self> {
        Self::try_from_string(input)
    }
}

impl fmt::Debug for CVV {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.masked_debug(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[cfg(test)]
    impl FromStr for CVV {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self> {
            Self::try_from(s.to_owned())
        }
    }
}
