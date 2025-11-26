use std::convert::TryFrom;
use std::fmt;
use zeroize_derive::ZeroizeOnDrop;

use crate::internal::{Masked, Validated};
use crate::{AsUnsafeRef, Error};

/// Tokenized credential from a payment processor or vault
///
/// # Validation
/// * length: 16-4096 characters,
/// * no leading or trailing spaces are allowed
///
/// # Data Protection
/// Tokens provide full access to payment credentials and enable unauthorized transactions,
/// making them sensitive authentication data (SAD).
///
/// As such, they are:
/// * fully masked in logs (via `Debug` implementation) to prevent any leaks,
/// * not exposed publicly except for a part of a request or response
///   via **unsafe** method `with_exposed_secret`.
#[derive(Clone, ZeroizeOnDrop)]
pub struct Token(String);

// Converters

impl<'a> TryFrom<&'a str> for Token {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self(input.to_string()).validate()
    }
}

impl AsUnsafeRef<str> for Token {
    #[inline]
    unsafe fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Debug for Token {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.masked_debug(f)
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Validated for Token {
    #[inline]
    fn validate(self) -> Result<Self, Error> {
        self._validate_length(&self.0, 16, 4096)?;
        self._validate_no_trailing_spaces(&self.0)?;
        Ok(self)
    }
}

// SAFETY: The trait is safely implemented as it does NOT expose any part of the token,
// fully protecting this sensitive authentication data from exposure in debug output.
unsafe impl Masked for Token {
    const TYPE_WRAPPER: &'static str = "Token";
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_TOKEN: &str = "1234567890123456";
    const VALID_TOKEN_LONG: &str = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ";

    mod construction {
        use super::*;

        #[test]
        fn accepts_valid_tokens() {
            for input in [
                VALID_TOKEN,
                VALID_TOKEN_LONG,
                "a".repeat(16).as_str(),
                "a".repeat(4096).as_str(),
            ] {
                let result = Token::try_from(input);
                assert!(result.is_ok(), "{input:?} failed validation");
            }
        }

        #[test]
        fn rejects_too_short_token() {
            let input = "123456789012345"; // 15 characters
            let result = Token::try_from(input);

            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_too_long_token() {
            let input = "a".repeat(4097);
            let result = Token::try_from(input.as_str());

            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_token_with_leading_space() {
            let input = " 1234567890123456";
            let result = Token::try_from(input);

            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_token_with_trailing_space() {
            let input = "1234567890123456 ";
            let result = Token::try_from(input);

            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }
    }

    mod safety {
        use super::*;

        #[test]
        fn masks_debug() {
            let token = Token::try_from(VALID_TOKEN).unwrap();
            let debug_output = format!("{:?}", token);
            assert!(debug_output.contains(r#"Token("***")"#));
            assert!(!debug_output.contains("1234"));
        }

        #[test]
        fn memory_is_not_leaked_after_drop() {
            let ptr: *const u8;
            let len: usize;
            unsafe {
                let token = Token::try_from(VALID_TOKEN).unwrap();
                let s = token.as_ref();
                ptr = s.as_ptr();
                len = s.len();
            }

            unsafe {
                let slice = std::slice::from_raw_parts(ptr, len);
                let original_bytes = VALID_TOKEN.as_bytes();
                assert_ne!(
                    slice, original_bytes,
                    "Original token should not remain in memory after drop"
                );
            }
        }
    }
}
