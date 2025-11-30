use std::convert::TryFrom;
use std::fmt;
use zeroize_derive::ZeroizeOnDrop;

use crate::internal::{Masked, Validated};
use crate::{AsUnsafeRef, Error};

/// Token returned after 3D Secure authentication
///
/// # Validation
/// * length: 16-4096 characters
///
/// # Data Protection
/// ThreeDSecureToken is a gateway-specific authentication token. Exposure could enable
/// replay attacks or unauthorized transaction attempts, making it sensitive authentication data.
///
/// As such, it is:
/// * fully masked in logs (via `Debug` implementation) to prevent any leaks,
/// * not exposed publicly except for a part of a request or response
///   via **unsafe** method `with_exposed_secret`.
#[derive(Clone, ZeroizeOnDrop)]
pub struct ThreeDSecureToken(String);

// Converters

impl<'a> TryFrom<&'a str> for ThreeDSecureToken {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self(input.to_string()).validate()
    }
}

impl AsUnsafeRef<str> for ThreeDSecureToken {
    #[inline]
    unsafe fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Debug for ThreeDSecureToken {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.masked_debug(f)
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Validated for ThreeDSecureToken {
    #[inline]
    fn validate(self) -> Result<Self, Error> {
        self._validate_length(&self.0, 16, 4096)?;
        Ok(self)
    }
}

// SAFETY: The trait is safely implemented as it does NOT expose any part of the token,
// fully protecting this sensitive authentication data from exposure in debug output.
unsafe impl Masked for ThreeDSecureToken {
    const TYPE_WRAPPER: &'static str = "ThreeDSecureToken";
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_TOKEN: &str = "abc123xyz789token";

    mod construction {
        use super::*;

        #[test]
        fn accepts_valid_token() {
            for input in [
                VALID_TOKEN,
                "a".repeat(16).as_str(),
                "a".repeat(4096).as_str(),
            ] {
                let result = ThreeDSecureToken::try_from(input);
                assert!(result.is_ok(), "{input:?} failed validation");
            }
        }

        #[test]
        fn rejects_too_short_token() {
            let input = "a".repeat(15);
            let result = ThreeDSecureToken::try_from(input.as_str());

            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_too_long_token() {
            let input = "a".repeat(4097);
            let result = ThreeDSecureToken::try_from(input.as_str());

            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }
    }

    mod safety {
        use super::*;

        #[test]
        fn masks_debug() {
            let token = ThreeDSecureToken::try_from(VALID_TOKEN).unwrap();
            let debug_output = format!("{:?}", token);
            assert!(debug_output.contains(r#"ThreeDSecureToken("***")"#));
            assert!(!debug_output.contains("abc123"));
        }

        #[test]
        fn memory_is_not_leaked_after_drop() {
            let ptr: *const u8;
            let len: usize;
            unsafe {
                let token = ThreeDSecureToken::try_from(VALID_TOKEN).unwrap();
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
