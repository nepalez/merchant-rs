use std::convert::TryFrom;
use std::fmt;

use crate::error::{Error, Result};
use crate::types::SecretString;

/// Minimum token length, typically 16 (to mimic PAN length).
const MIN_TOKEN_LENGTH: usize = 16;
/// Maximum token length to prevent Denial-of-Service (DoS) attacks via overly large input.
const MAX_TOKEN_LENGTH: usize = 4096;
/// Standard fixed mask prefix for logs.
const FIXED_MASK_PREFIX: &str = "********";

/// A Tokenized Payment Credential (e.g., from a payment processor or vault).
/// Wraps secrecy::SecretBox<String> to ensure memory is zeroed on drop and value is masked in Debug/Display.
#[derive(Clone)]
pub struct PaymentToken(SecretString);

impl PaymentToken {
    /// Exposes the first six characters of the token as a String.
    #[inline]
    pub fn first_six(&self) -> String {
        // SAFETY: Safe as the length of the token is 16+ characters, which is greater
        //         than a minimal length of PAN (13 characters) for which
        //         exposing first six digits is explicitly permitted by PCI DSS.
        unsafe { self.0.first_six() }
    }

    /// Exposes the last four characters of the token as a String.
    #[inline]
    pub fn last_four(&self) -> String {
        // SAFETY: Safe as the length of the token is 16+ characters, which is greater
        //         than a minimal length of PAN (13 characters) for which
        //         exposing last four digits is explicitly permitted by PCI DSS.
        unsafe { self.0.last_four() }
    }

    /// Returns the inner payment token string slice.
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

impl TryFrom<String> for PaymentToken {
    type Error = Error;

    #[inline]
    fn try_from(input: String) -> Result<Self> {
        validate(input.as_str())?;
        Ok(Self(input.into()))
    }
}

impl fmt::Debug for PaymentToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let masked_number = format!("{FIXED_MASK_PREFIX}{}", self.last_four());
        f.debug_tuple("PrimaryAccountNumber")
            .field(&masked_number)
            .finish()
    }
}

fn validate(input: &str) -> Result<()> {
    let len = input.len();

    if input.trim() != input {
        Err(Error::validation_failed(
            "Payment token contains invalid leading or trailing whitespace".to_string(),
        ))
    } else if len < MIN_TOKEN_LENGTH {
        Err(Error::validation_failed(format!(
            "Payment token length ({len}) is below the minimum required length ({}).",
            MIN_TOKEN_LENGTH
        )))
    } else if len > MAX_TOKEN_LENGTH {
        Err(Error::validation_failed(format!(
            "Payment token length ({len}) exceeds the maximum allowed length ({}).",
            MAX_TOKEN_LENGTH
        )))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    impl FromStr for PaymentToken {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self> {
            Self::try_from(s.to_owned())
        }
    }
}
