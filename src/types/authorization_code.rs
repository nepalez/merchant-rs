use std::convert::TryFrom;
use std::fmt;
use zeroize_derive::ZeroizeOnDrop;

use crate::internal::{Masked, Validated, sanitized::*};
use crate::{AsUnsafeRef, Error};

/// Authorization code from a card issuer
///
/// Supports both ISO 8583 standard (6 numeric digits)
/// and extended formats used by legacy/regional processors (e.g., older European
/// acquirers, some APAC processors use up to 8-10 characters).
///
/// # Sanitization
/// * removes common separators: spaces and dashes,
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * length: 6-10 characters,
/// * only alphanumeric characters are allowed
///
/// Gateway-specific validators should enforce stricter rules if necessary.
///
/// # Data Protection
/// While authorization codes are not Sensitive Authentication Data per PCI DSS,
/// they represent operational sensitive data. Defense-in-depth approach prevents
/// potential replay attacks in legacy systems and accidental exposure in logs.
///
/// As such, they are:
/// * masked in logs (via `Debug` implementation) to display
///   1 first and 1 last characters (both uppercased) only,
/// * not exposed publicly except for a part of a request or response
///   via **unsafe** method `with_exposed_secret`.
#[derive(Clone, ZeroizeOnDrop)]
pub struct AuthorizationCode(String);

impl<'a> TryFrom<&'a str> for AuthorizationCode {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self::sanitize(input).validate()
    }
}

impl fmt::Debug for AuthorizationCode {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.masked_debug(f)
    }
}

impl AsUnsafeRef<str> for AuthorizationCode {
    #[inline]
    unsafe fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for AuthorizationCode {
    #[inline]
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        trim_whitespaces(&mut output.0, input);
        output
    }
}

impl Validated for AuthorizationCode {
    fn validate(self) -> Result<Self, Error> {
        self._validate_length(&self.0, 6, 10)?;
        self._validate_alphanumeric(&self.0, "")?;
        Ok(self)
    }
}

// SAFETY: The trait is safely implemented because exposing the first 1 and last 1 character:
// 1. Neither causes out-of-bounds access to potentially INVALID (empty) data,
//    due to fallbacks to the empty strings,
// 2. Nor leaks the essential part of the sensitive VALID data
//    due to hiding the real length of the name.
unsafe impl Masked for AuthorizationCode {
    const TYPE_WRAPPER: &'static str = "AuthorizationCode";

    #[inline]
    fn first_chars(&self) -> String {
        self.0.get(0..1).unwrap_or_default().to_string()
    }

    #[inline]
    fn last_chars(&self) -> String {
        let len = self.0.len();
        self.0.get(len - 1..len).unwrap_or_default().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_CODE_6: &str = "123456";
    const VALID_CODE_8: &str = "ABC12345";
    const VALID_CODE_10: &str = "1234567890";

    mod construction {
        use super::*;

        #[test]
        fn accepts_valid_codes() {
            for input in [VALID_CODE_6, VALID_CODE_8, VALID_CODE_10] {
                let result = AuthorizationCode::try_from(input);
                assert!(result.is_ok(), "{input:?} failed validation");
            }
        }

        #[test]
        fn removes_separators() {
            let input = " 123456 \n\t\r ";
            let code = AuthorizationCode::try_from(input).unwrap();
            let result = unsafe { code.as_ref() };
            assert_eq!(result, VALID_CODE_6);
        }

        #[test]
        fn rejects_too_short_code() {
            let input = "12345"; // 5 characters
            let result = AuthorizationCode::try_from(input);

            if let Err(Error::InvalidInput(msg)) = result {
                assert!(msg.contains("1***5"));
            } else {
                panic!("Expected InvalidInput error, got {result:?}");
            }
        }

        #[test]
        fn rejects_too_long_code() {
            let input = "12345678901"; // 11 characters
            let result = AuthorizationCode::try_from(input);

            if let Err(Error::InvalidInput(msg)) = result {
                assert!(msg.contains("1***1"));
            } else {
                panic!("Expected InvalidInput error, got {result:?}");
            }
        }

        #[test]
        fn rejects_non_alphanumeric_characters() {
            let input = "12345@";
            let result = AuthorizationCode::try_from(input);

            if let Err(Error::InvalidInput(msg)) = result {
                assert!(msg.contains("1***@"));
            } else {
                panic!("Expected InvalidInput error, got {result:?}");
            }
        }
    }

    mod safety {
        use super::*;

        #[test]
        fn masks_debug() {
            let code = AuthorizationCode::try_from(VALID_CODE_6).unwrap();
            let debug_output = format!("{:?}", code);
            assert!(debug_output.contains(r#"AuthorizationCode("1***6")"#));
        }

        #[test]
        fn as_ref_is_unsafe() {
            static_assertions::assert_not_impl_all!(AuthorizationCode: AsRef<str>);

            let input = " 123456 \n\t";
            let code = AuthorizationCode::try_from(input).unwrap();
            let exposed = unsafe { <AuthorizationCode as AsUnsafeRef<str>>::as_ref(&code) };
            assert_eq!(exposed, VALID_CODE_6);
        }

        #[test]
        fn memory_is_not_leaked_after_drop() {
            let ptr: *const u8;
            let len: usize;
            unsafe {
                let code = AuthorizationCode::try_from(VALID_CODE_6).unwrap();
                let s = code.as_ref();
                ptr = s.as_ptr();
                len = s.len();
            }

            // SAFETY: This test verifies memory was zeroed after a drop.
            // Reading potentially freed memory is unsafe and only valid in tests
            // immediately after a drop, before any reallocation.
            unsafe {
                let slice = std::slice::from_raw_parts(ptr, len);
                let original_bytes = VALID_CODE_6.as_bytes();
                assert_ne!(
                    slice, original_bytes,
                    "Original authorization code should not remain in memory after drop"
                );
            }
        }
    }
}
