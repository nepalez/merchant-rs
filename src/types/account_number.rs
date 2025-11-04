use std::convert::TryFrom;
use std::fmt;
use zeroize_derive::ZeroizeOnDrop;

use crate::internal::{Masked, Validated, sanitized::*};
use crate::{AsUnsafeRef, Error};

/// Bank account number (for non-SEPA transfers)
///
/// For SEPA use a more specific type (`IBAN`).
///
/// # Sanitization
/// * removes common separators: spaces, dashes and underscores,
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * length: 4-20 characters,
/// * only alphanumeric characters are allowed
///
/// # Data Protection
/// While NOT classified as Sensitive Authentication Data (SAD) by PCI DSS,
/// account numbers provide direct access to bank accounts and enable unauthorized ACH/wire transfers,
/// making them critical financial access data analogous to PAN.
///
/// As such, they are:
/// * fully masked in logs (via `Debug` implementation) to prevent any leaks,
/// * not exposed publicly except for a part of a request or response
///   via **unsafe** method `with_exposed_secret`.
#[derive(Clone, ZeroizeOnDrop)]
pub struct AccountNumber(String);

impl<'a> TryFrom<&'a str> for AccountNumber {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self::sanitize(input).validate()
    }
}

impl fmt::Debug for AccountNumber {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as Masked>::masked_debug(self, f)
    }
}

impl AsUnsafeRef<str> for AccountNumber {
    #[inline]
    unsafe fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for AccountNumber {
    #[inline]
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        filter_characters(&mut output.0, input, "_-");
        output
    }
}

impl Validated for AccountNumber {
    #[inline]
    fn validate(self) -> Result<Self, Error> {
        self._validate_length(&self.0, 4, 20)?;
        self._validate_alphanumeric(&self.0, "")?;
        Ok(self)
    }
}

// SAFETY: The trait is safely implemented as it masks the total value in logs.
unsafe impl Masked for AccountNumber {
    const TYPE_WRAPPER: &'static str = "AccountNumber";
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_NUMBER: &str = "ABC123XYZ";

    mod construction {
        use super::*;

        #[test]
        fn accepts_valid_numbers() {
            for input in [VALID_NUMBER, "1234", "12345678901234567890"] {
                let result = AccountNumber::try_from(input);
                assert!(result.is_ok(), "{input:?} failed validation");
            }
        }

        #[test]
        fn removes_separators() {
            let input = " ABC1-2_3XYZ \n\t\r ";
            let account = AccountNumber::try_from(input).unwrap();
            let result = unsafe { account.as_ref() };
            assert_eq!(result, VALID_NUMBER);
        }

        #[test]
        fn rejects_too_short_number() {
            let input = "123"; // 3 characters
            let result = AccountNumber::try_from(input);

            if let Err(Error::InvalidInput(msg)) = result {
                assert!(msg.contains(r#"AccountNumber("***")"#));
            } else {
                panic!("Expected InvalidInput error, got {result:?}");
            }
        }

        #[test]
        fn rejects_too_long_number() {
            let input = "123456789012345678901"; // 21 characters
            let result = AccountNumber::try_from(input);

            if let Err(Error::InvalidInput(msg)) = result {
                assert!(msg.contains(r#"AccountNumber("***")"#));
            } else {
                panic!("Expected InvalidInput error, got {result:?}");
            }
        }

        #[test]
        fn rejects_non_alphanumeric_characters() {
            let input = "12345@6789";
            let result = AccountNumber::try_from(input);

            if let Err(Error::InvalidInput(msg)) = result {
                assert!(msg.contains(r#"AccountNumber("***")"#));
            } else {
                panic!("Expected InvalidInput error, got {result:?}");
            }
        }
    }

    mod safety {
        use super::*;

        #[test]
        fn masks_debug() {
            let account = AccountNumber::try_from(VALID_NUMBER).unwrap();
            let debug_output = format!("{:?}", account);
            assert!(debug_output.contains(r#"AccountNumber("***")"#));
        }

        #[test]
        fn as_ref_is_unsafe() {
            static_assertions::assert_not_impl_all!(AccountNumber: AsRef<str>);

            let input = "ABC-123-XYZ";
            let account = AccountNumber::try_from(input).unwrap();
            let exposed = unsafe { <AccountNumber as AsUnsafeRef<str>>::as_ref(&account) };
            assert_eq!(exposed, VALID_NUMBER);
        }

        #[test]
        fn memory_is_not_leaked_after_drop() {
            let ptr: *const u8;
            let len: usize;
            unsafe {
                let account = AccountNumber::try_from(VALID_NUMBER).unwrap();
                let s = account.as_ref();
                ptr = s.as_ptr();
                len = s.len();
            }

            // SAFETY: This test verifies memory was zeroed after a drop.
            // Reading potentially freed memory is unsafe and only valid in tests
            // immediately after a drop, before any reallocation.
            unsafe {
                let slice = std::slice::from_raw_parts(ptr, len);
                let original_bytes = VALID_NUMBER.as_bytes();
                assert_ne!(
                    slice, original_bytes,
                    "Original account number should not remain in memory after drop"
                );
            }
        }
    }
}
