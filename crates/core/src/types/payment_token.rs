use std::convert::TryFrom;
use std::fmt;

use crate::error::{Error, Result};
use crate::types::{SafeWrapper, Sanitized, SecretString, Validated};

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
    /// Exposes the last four characters of the token as a String.
    #[inline]
    pub fn last_four(&self) -> String {
        // SAFETY: Safe as the length of the token is 16+ characters, which is greater
        //         than a minimal length of PAN (13 characters) for which
        //         exposing last four digits is explicitly permitted by PCI DSS.
        unsafe { self.0.last_chars(4) }.to_owned()
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
        // SAFETY: the safety contract is passed to the caller.
        unsafe { self.0.with_exposed_secret(f) }
    }
}

impl TryFrom<String> for PaymentToken {
    type Error = Error;

    #[inline]
    fn try_from(input: String) -> Result<Self> {
        Self::try_from_string(input)
    }
}

impl fmt::Debug for PaymentToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // SAFETY: Explicitly enabled by PCI DSS for PANs which are even shorter than tokens.
        let last_four = unsafe { self.0.last_chars(4) };
        let masked_number = format!("{FIXED_MASK_PREFIX}{}", last_four);
        f.debug_tuple("PrimaryAccountNumber")
            .field(&masked_number)
            .finish()
    }
}

// Sealed traits implementations

// The token should never be modified. Any inconsistency is treated as an error.
impl Sanitized for PaymentToken {}

impl Validated for PaymentToken {
    const TYPE_NAME: &'static str = "Payment token";
    const MIN_LENGTH: usize = 16;
    const MAX_LENGTH: usize = 4096;
    const EXTRA_CHARS: Option<&'static str> = None;

    #[inline]
    fn validate(input: &str) -> Result<()> {
        if input.trim() != input {
            return Err(Error::validation_failed(format!(
                "{} contains invalid leading or trailing whitespace",
                Self::TYPE_NAME
            )));
        }

        Self::validate_length(input)?;

        Ok(())
    }
}

impl SafeWrapper for PaymentToken {
    type Inner = String;

    fn wrap(inner: Self::Inner) -> Self {
        Self(inner.into())
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
