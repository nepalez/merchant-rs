use std::convert::TryFrom;
use std::fmt;
use zeroize_derive::ZeroizeOnDrop;

use crate::internal::{Masked, Validated};
use crate::{AsUnsafeRef, Error};

/// Cardholder Authentication Verification Value from 3D Secure authentication
///
/// # Validation
/// * length: 27-44 characters (Base64-encoded cryptogram)
///
/// # Data Protection
/// CAVV is a cryptographic proof of cardholder authentication. Exposure could enable
/// replay attacks or fraud analysis, making it sensitive authentication data (SAD).
///
/// As such, it is:
/// * fully masked in logs (via `Debug` implementation) to prevent any leaks,
/// * not exposed publicly except for a part of a request or response
///   via **unsafe** method `with_exposed_secret`.
#[derive(Clone, ZeroizeOnDrop)]
pub struct CAVV(String);

// Converters

impl<'a> TryFrom<&'a str> for CAVV {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self(input.to_string()).validate()
    }
}

impl AsUnsafeRef<str> for CAVV {
    #[inline]
    unsafe fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Debug for CAVV {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.masked_debug(f)
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Validated for CAVV {
    #[inline]
    fn validate(self) -> Result<Self, Error> {
        self._validate_length(&self.0, 27, 44)?;
        Ok(self)
    }
}

// SAFETY: The trait is safely implemented as it does NOT expose any part of the CAVV,
// fully protecting this sensitive authentication data from exposure in debug output.
unsafe impl Masked for CAVV {
    const TYPE_WRAPPER: &'static str = "CAVV";
}

#[cfg(test)]
mod tests {
    use super::*;

    // 28 characters - valid Base64-encoded CAVV
    const VALID_CAVV: &str = "AAABBWFlmQAAAABjRWWZEEFgFz8=";

    mod construction {
        use super::*;

        #[test]
        fn accepts_valid_cavv() {
            for input in [VALID_CAVV, "a".repeat(27).as_str(), "a".repeat(44).as_str()] {
                let result = CAVV::try_from(input);
                assert!(result.is_ok(), "{input:?} failed validation");
            }
        }

        #[test]
        fn rejects_too_short_cavv() {
            let input = "a".repeat(26);
            let result = CAVV::try_from(input.as_str());

            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_too_long_cavv() {
            let input = "a".repeat(45);
            let result = CAVV::try_from(input.as_str());

            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }
    }

    mod safety {
        use super::*;

        #[test]
        fn masks_debug() {
            let cavv = CAVV::try_from(VALID_CAVV).unwrap();
            let debug_output = format!("{:?}", cavv);
            assert!(debug_output.contains(r#"CAVV("***")"#));
            assert!(!debug_output.contains("AAAB"));
        }

        #[test]
        fn memory_is_not_leaked_after_drop() {
            let ptr: *const u8;
            let len: usize;
            unsafe {
                let cavv = CAVV::try_from(VALID_CAVV).unwrap();
                let s = cavv.as_ref();
                ptr = s.as_ptr();
                len = s.len();
            }

            unsafe {
                let slice = std::slice::from_raw_parts(ptr, len);
                let original_bytes = VALID_CAVV.as_bytes();
                assert_ne!(
                    slice, original_bytes,
                    "Original CAVV should not remain in memory after drop"
                );
            }
        }
    }
}
