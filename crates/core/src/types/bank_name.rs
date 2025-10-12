// bank_name.rs
use std::convert::TryFrom;

use crate::error::{Error, Result};
use crate::types::{SafeWrapper, Sanitized, Validated};

const MAX_LENGTH: usize = 100;
const TYPE_NAME: &str = "Bank name";

/// Legal name of the financial institution.
///
/// Max length 100: Set for robust internal storage of long legal bank names
/// (e.g., "The Hongkong and Shanghai Banking Corporation Limited").
/// Character set follows ISO 20022 / SWIFT standards for bank names, allowing
/// alphanumeric, spaces, and common punctuation (-, ., ,, ', &, parentheses, /).
/// The slash is included for branch designations used in some jurisdictions
/// (e.g., "HSBC Bank USA, N.A. / California Branch").
/// Sanitization: Only trims. Full legal name should be kept intact for internal use;
/// reliance is on validation.
#[derive(Clone, Debug)]
pub struct BankName(String);

impl TryFrom<String> for BankName {
    type Error = Error;

    #[inline]
    fn try_from(input: String) -> Result<Self> {
        Self::try_from_string(input)
    }
}

// Sealed traits implementations

impl Sanitized for BankName {
    fn sanitize(input: String) -> Result<String> {
        let trimmed = input.trim();
        if trimmed.len() == input.len() {
            Ok(input)
        } else {
            Ok(trimmed.to_string())
        }
    }
}

impl Validated for BankName {
    fn validate(input: &str) -> Result<()> {
        let len = input.len();

        if len == 0 {
            Err(Error::validation_failed(format!(
                "{TYPE_NAME} cannot be empty"
            )))
        } else if len > MAX_LENGTH {
            Err(Error::validation_failed(format!(
                "{TYPE_NAME} length ({len}) exceeds maximum ({MAX_LENGTH})"
            )))
        } else if !input.chars().all(|c| {
            c.is_ascii_alphanumeric()
                || matches!(c, ' ' | '-' | '.' | ',' | '\'' | '&' | '(' | ')' | '/')
        }) {
            Err(Error::validation_failed(format!(
                "{TYPE_NAME} must contain only alphanumeric characters, spaces, \
                        and standard punctuation (-, ., ,, ', &, parentheses, /)"
            )))
        } else {
            Ok(())
        }
    }
}

impl SafeWrapper for BankName {
    type Inner = String;

    fn wrap(inner: String) -> Self {
        Self(inner)
    }
}
