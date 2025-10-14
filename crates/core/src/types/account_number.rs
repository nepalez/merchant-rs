use std::convert::TryFrom;
use std::fmt;

use crate::error::{Error, Result};
use crate::internal::*;

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

// SAFETY
//
// The trait is safely implemented because:
// 1. The wrapper uses SecretString as inner type, which guarantees memory zeroization on drop.
// 2. No characters are exposed via `Debug` or `Display`, preventing accidental leakage.
unsafe impl SafeWrapper for AccountNumber {
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

impl Sanitized for AccountNumber {
    const CHARS_TO_REMOVE: Option<&'static str> = Some("-_");
}

impl Validated for AccountNumber {
    const TYPE_NAME: &'static str = "AccountNumber";
    const MIN_LENGTH: usize = 4;
    const MAX_LENGTH: usize = 20;
    // Checks that the account number contains alphanumeric characters only.
    const EXTRA_CHARS: Option<&'static str> = Some("");
}

impl TryFrom<String> for AccountNumber {
    type Error = Error;

    #[inline]
    fn try_from(input: String) -> Result<Self> {
        Self::try_from_string(input)
    }
}

impl fmt::Debug for AccountNumber {
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
    impl FromStr for AccountNumber {
        type Err = Error;

        fn from_str(s: &str) -> Result<Self> {
            Self::try_from(s.to_owned())
        }
    }
}
