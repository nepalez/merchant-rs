use std::fmt;
use zeroize_derive::ZeroizeOnDrop;

use crate::internal::{Masked, Validated, sanitized::*};
use crate::{AsUnsafeRef, Error};

/// Card Verification Value (CVV/CVC/CID)
///
/// # Sanitization
/// * trims whitespaces,
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * length: 3-4 characters,
/// * only digits are allowed
///
/// # Data Protection
/// PCI DSS classifies CVV as sensitive authentication data (SAD) that verifies
/// physical card possession and prevents card-not-present fraud.
///
/// As such, it is:
/// * fully masked in logs (via `Debug` implementation) to prevent any leaks,
/// * not exposed publicly except for a part of a request or response
///   via **unsafe** method `with_exposed_secret`.
#[derive(Clone, ZeroizeOnDrop)]
#[allow(clippy::upper_case_acronyms)]
pub struct CVV(String);

impl<'a> TryFrom<&'a str> for CVV {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self::sanitize(input).validate()
    }
}

impl fmt::Debug for CVV {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as Masked>::masked_debug(self, f)
    }
}

impl AsUnsafeRef<str> for CVV {
    #[inline]
    unsafe fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for CVV {
    #[inline]
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        for c in input.trim().chars() {
            if !c.is_ascii_control() {
                output.0.push(c);
            }
        }
        output
    }
}

impl Validated for CVV {
    #[inline]
    fn validate(self) -> Result<Self, Error> {
        self._validate_length(&self.0, 3, 4)?;
        self._validate_digits(&self.0, "")?;
        Ok(self)
    }
}

/// # Safety
/// The trait is safely implemented as it does NOT expose any part of CVV,
/// fully protecting this sensitive authentication data in all contexts.
unsafe impl Masked for CVV {
    const TYPE_WRAPPER: &'static str = "CVV";
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_CVV_3: &str = "123";
    const VALID_CVV_4: &str = "1234";

    mod construction {
        use super::*;

        #[test]
        fn accepts_valid_numbers() {
            for input in [VALID_CVV_3, VALID_CVV_4] {
                let result = CVV::try_from(input);
                assert!(result.is_ok(), "{input:?} failed validation");
            }
        }

        #[test]
        fn removes_control_characters() {
            let input = " 123\n\t\r ";
            let cvv = CVV::try_from(input).unwrap();
            let result = unsafe { cvv.as_ref() };
            assert_eq!(result, VALID_CVV_3);
        }

        #[test]
        fn rejects_too_short_number() {
            let input = "12"; // 2 digits
            let result = CVV::try_from(input);

            if let Err(Error::InvalidInput(msg)) = result {
                assert!(msg.contains(r#"CVV("***")"#));
            } else {
                panic!("Expected InvalidInput error, got {result:?}");
            }
        }

        #[test]
        fn rejects_too_long_number() {
            let input = "12345"; // 5 digits
            let result = CVV::try_from(input);

            if let Err(Error::InvalidInput(msg)) = result {
                assert!(msg.contains(r#"CVV("***")"#));
            } else {
                panic!("Expected InvalidInput error, got {result:?}");
            }
        }

        #[test]
        fn rejects_non_numeric_characters() {
            let input = "12A";
            let result = CVV::try_from(input);

            if let Err(Error::InvalidInput(msg)) = result {
                assert!(msg.contains(r#"CVV("***")"#));
            } else {
                panic!("Expected InvalidInput error, got {result:?}");
            }
        }
    }

    mod safety {
        use super::*;

        #[test]
        fn masks_debug() {
            let cvv = CVV::try_from(VALID_CVV_3).unwrap();
            let debug_output = format!("{:?}", cvv);
            assert!(debug_output.contains(r#"CVV("***")"#));
        }

        #[test]
        fn as_ref_is_unsafe() {
            static_assertions::assert_not_impl_all!(CVV: AsRef<str>);

            let input = " 123 \n\t";
            let cvv = CVV::try_from(input).unwrap();
            let exposed = unsafe { <CVV as AsUnsafeRef<str>>::as_ref(&cvv) };
            assert_eq!(exposed, VALID_CVV_3);
        }

        #[test]
        fn memory_is_not_leaked_after_drop() {
            let ptr: *const u8;
            let len: usize;
            unsafe {
                let cvv = CVV::try_from(VALID_CVV_3).unwrap();
                let s = cvv.as_ref();
                ptr = s.as_ptr();
                len = s.len();
            }

            // SAFETY: This test verifies memory was zeroed after a drop.
            // Reading potentially freed memory is unsafe and only valid in tests
            // immediately after a drop, before any reallocation.
            unsafe {
                let slice = std::slice::from_raw_parts(ptr, len);
                let original_bytes = VALID_CVV_3.as_bytes();
                assert_ne!(
                    slice, original_bytes,
                    "Original CVV should not remain in memory after drop"
                );
            }
        }
    }
}
