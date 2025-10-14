// bank_name.rs
use std::convert::TryFrom;

use crate::error::*;
use crate::internal::*;

/// Legal name of the financial institution.
///
/// # Input Constraints
/// Max length 100: Set for robust internal storage of long legal bank names
/// (e.g., "The Hongkong and Shanghai Banking Corporation Limited").
/// Min length 2: Rejects accidental single-character input while accommodating
/// short bank names (e.g., "N26", Chinese bank abbreviations like "MB").
/// Character set follows ISO 20022 / SWIFT standards for bank names, allowing
/// alphanumeric, spaces, and common punctuation (-, ., ,, ', &, parentheses, /).
/// The slash is included for branch designations used in some jurisdictions
/// (e.g., "HSBC Bank USA, N.A. / California Branch").
///
/// Sanitization: Only trims. Full legal name should be kept intact for internal use;
/// reliance is on validation.
#[derive(Clone, Debug)]
pub struct BankName(String);

// SAFETY: The trait is safely implemented because this type is not considered sensitive data.
unsafe impl SafeWrapper for BankName {
    type Inner = String;

    #[inline]
    fn wrap(inner: Self::Inner) -> Self {
        Self(inner)
    }

    #[inline]
    unsafe fn inner(&self) -> &Self::Inner {
        &self.0
    }
}

impl Sanitized for BankName {
    const TRIM: bool = true;
}

impl Validated for BankName {
    const TYPE_NAME: &'static str = "BankName";
    const MIN_LENGTH: usize = 2;
    const MAX_LENGTH: usize = 100;
    const EXTRA_CHARS: Option<&'static str> = Some(" -.,\'&()/");
}

impl TryFrom<String> for BankName {
    type Error = Error;

    #[inline]
    fn try_from(input: String) -> Result<Self> {
        Self::try_from_string(input)
    }
}
