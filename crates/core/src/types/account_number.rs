use std::convert::TryFrom;
use std::fmt;

use crate::error::{Error, Result};
use crate::internal::*;

/// List of allowed separators in account number input strings.
const NUMBER_SEPARATORS: [char; 3] = [' ', '-', '_'];
/// Standard fixed mask for logs.
const FIXED_MASK: &str = "********";

/// Represents a bank account number, securely stored and validated.
///
/// # Regional Variability and Standards
///
/// Account numbers vary significantly by region and banking system:
/// - **IBAN (ISO 13616)**: Alphanumeric, up to 34 characters (e.g., "DE89370400440532013000")
/// - **US/Japan/Australia**: Typically numeric only (e.g., "1234567890")
/// - **SEPA/Europe**: IBAN format (alphanumeric)
///
/// This type accepts alphanumeric characters (A-Z, 0-9) after removing common separators
/// (spaces, hyphens, underscores) to accommodate international standards including IBAN.
/// Gateway-specific validators MUST enforce stricter rules where applicable
/// (e.g., numeric-only for US ACH transfers).
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
        Self::try_from_string(input)
    }
}

impl fmt::Debug for AccountNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("AccountNumber").field(&FIXED_MASK).finish()
    }
}

// Sealed traits implementations

impl SafeWrapper for AccountNumber {
    type Inner = SecretString;

    fn wrap(inner: SecretString) -> Self {
        Self(inner)
    }
}

impl Sanitized for AccountNumber {
    const CHARS_TO_REMOVE: Option<&'static str> = Some("-_");
}

impl Validated for AccountNumber {
    const TYPE_NAME: &'static str = "Account Number";
    const MIN_LENGTH: usize = 4;
    const MAX_LENGTH: usize = 20;
    // Checks that the account number contains alphanumeric characters only.
    const EXTRA_CHARS: Option<&'static str> = Some("");
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
