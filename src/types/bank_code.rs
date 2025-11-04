use std::convert::{AsRef, TryFrom};
use zeroize_derive::ZeroizeOnDrop;

use crate::Error;
use crate::internal::{Validated, sanitized::*};

/// Code identifying the bank
///
/// # Sanitization
/// * removes characters used in financial systems for formatting: spaces, dashes, dots, slashes,
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * length: 2-34 characters,
/// * only alphanumeric characters are allowed
///
/// The adapter implementation can apply stricter validation rules later.
///
/// # Data Protection
/// This is a public value, neither secret nor even PII.
///
/// Consequently, both `Debug` and `AsRef` are implemented without masking.
#[derive(Clone, Debug, ZeroizeOnDrop)]
pub struct BankCode(String);

impl<'a> TryFrom<&'a str> for BankCode {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Error> {
        Self::sanitize(input).validate()
    }
}

impl AsRef<str> for BankCode {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for BankCode {
    #[inline]
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        filter_characters(&mut output.0, input, ".-/");
        output
    }
}

impl Validated for BankCode {
    #[inline]
    fn validate(self) -> Result<Self, Error> {
        self._validate_length(&self.0, 2, 34)?;
        self._validate_alphanumeric(&self.0, "")?;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_CODE: &str = "12345678";
    const VALID_SWIFT: &str = "DEUTDEFF";

    mod construction {
        use super::*;

        #[test]
        fn accepts_valid_codes() {
            for input in [VALID_CODE, VALID_SWIFT, "AB", "12345678901234567890123456789012"] {
                let result = BankCode::try_from(input);
                assert!(result.is_ok(), "{input:?} failed validation");
            }
        }

        #[test]
        fn removes_separators() {
            let input = " 1234.5678 \n\t\r ";
            let code = BankCode::try_from(input).unwrap();
            let result = code.as_ref();
            assert_eq!(result, VALID_CODE);
        }

        #[test]
        fn rejects_too_short_code() {
            let input = "1"; // 1 character
            let result = BankCode::try_from(input);

            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_too_long_code() {
            let input = "123456789012345678901234567890123456"; // 36 characters
            let result = BankCode::try_from(input);

            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_non_alphanumeric_characters() {
            let input = "DEUT@DEFF";
            let result = BankCode::try_from(input);

            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }
    }
}
