use luhn3;
use std::convert::TryFrom;
use std::fmt;

use crate::error::{Error, Result};
use crate::types::{SafeWrapper, Sanitized, SecretString, Validated};

/// List of allowed separators in a PAN input strings.
const NUMBER_SEPARATORS: [char; 3] = [' ', '-', '_'];
/// Standard fixed mask prefix for logs.
const FIXED_MASK_PREFIX: &str = "********";

/// Represents a Primary Account Number (PAN),
/// securely stored and validated against basic invariants.
///
/// ## Invariants enforced
///
/// The implementation of `TryFrom<String>` ensures that the PAN:
/// * Contains only digits after successful cleaning and validation.
/// * Passes a minimal, basic structural check (universal length and MII prefix).
/// * Passes the Luhn check (Mod 10).
///
/// ## Invariants NOT enforced
///
/// Full, comprehensive, and up-to-date BIN validation is explicitly **NOT** performed.
/// It is the responsibility of the Application Layer.
///
/// # Security Considerations
///
/// * Uses secure storage to ensure memory zeroization after use.
/// * `Clone` is implemented for request resilience, but the cloned value
///   is immediately re-wrapped in a new secure container.
/// * `Debug` implementation masks all but the last four digits as permitted by PCI DSS.
///   The fixed mask prefix hides the length of the PAN to prevent leakage.
/// * `Display` is not implemented to prevent accidental leakage,
///   but access to the first six and last four digits is provided via methods,
///   as permitted by PCI DSS.
/// * Full access is only possible via the **unsafe** `with_exposed_secret` method,
///   which forces developers to acknowledge the handling of sensitive data.
///
/// # SAFETY
///
/// It is the responsibility of the caller to ensure
/// that the input string has been sourced securely
/// and has neither been cloned nor logged
/// before the construction of the `PrimaryAccountNumber`.
#[derive(Clone)]
pub struct PrimaryAccountNumber(SecretString);

impl PrimaryAccountNumber {
    /// Exposes the first six digits of the PAN as a String.
    #[inline]
    pub fn first_six(&self) -> String {
        // SAFETY: Safe as it its explicitly enabled by PCI DSS for PANs.
        unsafe { self.0.first_chars(6) }.to_owned()
    }

    /// Exposes the last four digits of the PAN as a String.
    pub fn last_four(&self) -> String {
        // SAFETY: Safe as it its explicitly enabled by PCI DSS for PANs.
        unsafe { self.0.last_chars(4) }.to_owned()
    }

    /// Exposes the underlying Primary Account Number (PAN) as a string slice.
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

impl TryFrom<String> for PrimaryAccountNumber {
    type Error = Error;

    #[inline]
    fn try_from(input: String) -> Result<Self> {
        Self::try_from_string(input)
    }
}

impl fmt::Debug for PrimaryAccountNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Safety: Safe as it its explicitly enabled by PCI DSS for PANs
        let last_four = unsafe { self.0.last_chars(4) };
        let masked_number = format!("{FIXED_MASK_PREFIX}{}", last_four);
        f.debug_tuple("PrimaryAccountNumber")
            .field(&masked_number)
            .finish()
    }
}

// Sealed traits implementations

impl Sanitized for PrimaryAccountNumber {
    fn sanitize(input: String) -> Result<String> {
        let mut output = String::with_capacity(input.len());

        for c in input.chars() {
            if c.is_ascii_digit() {
                output.push(c);
            } else if NUMBER_SEPARATORS.contains(&c) {
                continue;
            } else {
                return Err(Error::validation_failed(format!(
                    "Input contains invalid character '{c}'. \
                    Only digits, spaces, underscores, and hyphens are allowed."
                )));
            }
        }

        Ok(output)
    }
}

impl Validated for PrimaryAccountNumber {
    const TYPE_NAME: &'static str = "PAN";
    const MIN_LENGTH: usize = 13;
    const MAX_LENGTH: usize = 19;
    const EXTRA_CHARS: Option<&'static str> = None;

    #[inline]
    fn validate(input: &str) -> Result<()> {
        Self::validate_length(input)?;

        if input.starts_with('0') {
            return Err(Error::validation_failed(format!(
                "{} cannot start with '0'",
                Self::TYPE_NAME
            )));
        }

        if !luhn3::valid(input.as_bytes()) {
            return Err(Error::validation_failed(format!(
                "{} failed the Luhn check",
                Self::TYPE_NAME
            )));
        }

        Ok(())
    }
}

impl SafeWrapper for PrimaryAccountNumber {
    type Inner = SecretString;

    fn wrap(inner: SecretString) -> Self {
        Self(inner)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    impl FromStr for PrimaryAccountNumber {
        type Err = Error;

        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            Self::try_from(s.to_owned())
        }
    }
}
