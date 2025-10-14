use std::convert::TryFrom;
use std::fmt;

use crate::error::*;
use crate::internal::*;

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

// SAFETY:
//
// The trait is safely implemented because:
// 1. The wrapper uses SecretString as inner type, which guarantees memory zeroization on drop.
// 2. The Debug implementation masks all but the first 6 and the last 4 characters of the PAN,
//    which is explicitly allowed by PCI DSS for Primary Account Numbers (PANs).
// 3. The validation ensures that the PAN has at least 13 characters, so it is guaranteed
//    to have at least 6 characters to show from every side.
unsafe impl SafeWrapper for PrimaryAccountNumber {
    type Inner = SecretString;

    const FIRST_CHARS: usize = 6;
    const LAST_CHARS: usize = 4;

    #[inline]
    fn wrap(inner: Self::Inner) -> Self {
        Self(inner)
    }

    #[inline]
    unsafe fn inner(&self) -> &Self::Inner {
        &self.0
    }
}

impl Sanitized for PrimaryAccountNumber {
    const CHARS_TO_REMOVE: Option<&'static str> = Some("-_");
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

impl TryFrom<String> for PrimaryAccountNumber {
    type Error = Error;

    #[inline]
    fn try_from(input: String) -> Result<Self> {
        Self::try_from_string(input)
    }
}

impl fmt::Debug for PrimaryAccountNumber {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.masked_debug(f)
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
