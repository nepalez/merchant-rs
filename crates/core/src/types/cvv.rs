use std::convert::TryFrom;
use std::fmt;

use crate::error::{Error, Result};
use crate::types::SecretString;

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
        // Safety: the safety contract is passed to the caller.
        unsafe { self.0.with_exposed_secret(f) }
    }
}

impl TryFrom<String> for CVV {
    type Error = Error;

    #[inline]
    fn try_from(input: String) -> Result<Self> {
        let value = sanitize(input);
        validate(&value)?;
        Ok(Self(value.into()))
    }
}

impl fmt::Debug for CVV {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("AccountNumber").field(&FIXED_MASK).finish()
    }
}

fn sanitize(input: String) -> String {
    input
}

fn validate(input: &str) -> Result<()> {
    let len = input.len();

    if len < 3 {
        Err(Error::validation_failed(format!(
            "CVV length is too short: {}. Minimum length is 3.",
            len
        )))
    } else if len > 4 {
        Err(Error::validation_failed(format!(
            "CVV length is too long: {}. Maximum length is 4.",
            len
        )))
    } else if !input.chars().all(|c| c.is_ascii_digit()) {
        Err(Error::validation_failed(
            "CVV must contain only digits (0-9)".to_string(),
        ))
    } else {
        Ok(())
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
