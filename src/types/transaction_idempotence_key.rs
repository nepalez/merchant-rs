use std::convert::{AsRef, TryFrom};
use zeroize_derive::ZeroizeOnDrop;

use crate::Error;
use crate::internal::{Validated, sanitized::*};

/// Merchant's internal reference identifier for a transaction
///
/// # Sanitization
/// * trims whitespaces,
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * length: 1-255 characters
///
/// # Data Protection
/// This identifier is specifically designed for public usage and does not contain sensitive information.
///
/// Consequently, both `Debug` and `Display` are implemented without masking.
#[derive(Clone, Debug, ZeroizeOnDrop)]
pub struct TransactionIdempotenceKey(String);

impl<'a> TryFrom<&'a str> for TransactionIdempotenceKey {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self::sanitize(input).validate()
    }
}

impl AsRef<str> for TransactionIdempotenceKey {
    #[inline]
    fn as_ref(&self) -> &str {
        &self.0
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for TransactionIdempotenceKey {
    #[inline]
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        trim_whitespaces(&mut output.0, input);
        output
    }
}

impl Validated for TransactionIdempotenceKey {
    #[inline]
    fn validate(self) -> Result<Self, Error> {
        self._validate_length(&self.0, 1, 255)?;
        Ok(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_KEY: &str = "unique-key-123";

    mod construction {
        use super::*;

        #[test]
        fn accepts_valid_keys() {
            for input in [VALID_KEY, "a", "a".repeat(255).as_str()] {
                let result = TransactionIdempotenceKey::try_from(input);
                assert!(result.is_ok(), "{input:?} failed validation");
            }
        }

        #[test]
        fn removes_control_characters() {
            let input = " unique-key-123 \n\t\r ";
            let key = TransactionIdempotenceKey::try_from(input).unwrap();
            let result = key.as_ref();
            assert_eq!(result, VALID_KEY);
        }

        #[test]
        fn rejects_empty_key() {
            let result = TransactionIdempotenceKey::try_from("");
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_too_long_key() {
            let input = "a".repeat(256);
            let result = TransactionIdempotenceKey::try_from(input.as_str());
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }
    }

    mod safety {
        use super::*;

        #[test]
        fn exposes_debug() {
            let key = TransactionIdempotenceKey::try_from(VALID_KEY).unwrap();
            let debug_output = format!("{:?}", key);
            // This is public data, not masked
            assert!(debug_output.contains(VALID_KEY));
        }
    }
}
