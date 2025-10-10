use std::convert::TryFrom;
use std::fmt;

use crate::error::{Error, Result};
use crate::types::SecretString;

/// List of allowed separators in account number input strings.
const NUMBER_SEPARATORS: [char; 3] = [' ', '-', '_'];
/// The minimum required length for a bank account number (digits only).
const MIN_ACCOUNT_LENGTH: usize = 4;
/// The maximum allowed length for a bank account number (digits only).
const MAX_ACCOUNT_LENGTH: usize = 20;
/// Standard fixed mask for logs.
const FIXED_MASK: &str = "********";

/// Represents a bank account number, securely stored and validated.
/// Validation is minimal due to extreme regional variability.
///
/// # SAFETY
///
/// While the Account Number is NOT classified as Sensitive Authentication Data (SAD) by PCI DSS,
/// it is critical **Personally Identifiable Information (PII)**
/// and financial access data (used for ACH/wire transfers).
/// To enforce Defense-in-Depth, ensure guaranteed log masking,
/// and prevent accidental data leakage, it is treated with the same rigor as other sensitive data.
///
/// * The memory is forcefully zeroed on drop (guaranteed by SecretBox).
/// * The value is masked in `Debug` for log safety.
/// * Cloning is allowed for request resilience, but the cloned value
///   is immediately re-wrapped in a new `SecretBox`.
/// * The value can only be accessed via the **unsafe** `with_exposed_secret` method,
///   which forces developers to acknowledge the handling of sensitive financial PII.
#[derive(Clone)]
pub struct AccountNumber(SecretString);

impl AccountNumber {
    /// Exposes the underlying Account Number as a string slice.
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

impl TryFrom<String> for AccountNumber {
    type Error = Error;

    #[inline]
    fn try_from(input: String) -> Result<Self> {
        let number = sanitize(input)?;
        validate(number.as_str())?;
        Ok(Self(number.into()))
    }
}

impl fmt::Debug for AccountNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("AccountNumber").field(&FIXED_MASK).finish()
    }
}

fn sanitize(input: String) -> Result<String> {
    let mut cleaned_number = String::with_capacity(input.len());

    for c in input.chars() {
        if c.is_ascii_digit() {
            cleaned_number.push(c);
        } else if NUMBER_SEPARATORS.contains(&c) {
            continue;
        } else {
            return Err(Error::validation_failed(format!(
                "Input contains invalid character '{c}'.\
                     Only digits, spaces, and hyphens are allowed.",
            )));
        }
    }

    Ok(cleaned_number)
}

fn validate(sanitized_input: &str) -> Result<()> {
    let len = sanitized_input.len();

    // Account numbers widely vary, but must have some digits.
    if len < MIN_ACCOUNT_LENGTH {
        Err(Error::validation_failed(format!(
            "Account Number length ({len}) is below the minimum required length ({} digits).",
            MIN_ACCOUNT_LENGTH
        )))
    } else if len > MAX_ACCOUNT_LENGTH {
        Err(Error::validation_failed(format!(
            "Account Number length ({len}) exceeds the maximum allowed length ({} digits).",
            MAX_ACCOUNT_LENGTH
        )))
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[cfg(test)]
    impl FromStr for AccountNumber {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self> {
            Self::try_from(s.to_owned())
        }
    }
}
