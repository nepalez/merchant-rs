use iban::{Iban, IbanLike};
use std::fmt;
use std::str::FromStr;
use zeroize_derive::ZeroizeOnDrop;

use crate::internal::{Masked, Validated, sanitized::*};
use crate::{AsUnsafeRef, Error};

/// International Bank Account Number (IBAN)
///
/// # Sanitization
/// * removes common separators: spaces, dashes, slashes, dots and apostrophes,
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * validates against the [ISO 13616](https://en.wikipedia.org/wiki/ISO_13616) standard
///   via the crate [iban_validate](https://crates.io/crates/iban_validate)
///
/// # Data Protection
/// IBAN provides direct access to bank accounts and enables unauthorized withdrawals,
/// making them sensitive authentication data (SAD).
///
/// As such, it is:
/// * masked in logs (via `Debug` implementation) to display
///   the first 2 characters (country code) and last 4 characters only,
/// * not exposed publicly except for a part of a request or response
///   via **unsafe** method `with_exposed_secret`.
#[derive(Clone, ZeroizeOnDrop)]
#[allow(clippy::upper_case_acronyms)]
pub struct IBAN(String);

impl<'a> TryFrom<&'a str> for IBAN {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self::sanitize(input).validate()
    }
}

impl fmt::Debug for IBAN {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.masked_debug(f)
    }
}

impl AsUnsafeRef<str> for IBAN {
    #[inline]
    unsafe fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for IBAN {
    #[inline]
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        filter_characters(&mut output.0, input, ".-/");
        output
    }
}

impl Validated for IBAN {
    // TODO: strictly speaking, a zeroization of the resulting Iban is not enough
    //       because the `iban_validate` crate leaks some intermediate strings
    //       under the hood of its data validations.
    fn validate(self) -> Result<Self, Error> {
        let secret = Iban::from_str(&self.0)
            .map(Secret)
            .map_err(|_| Error::InvalidInput(format!("{self:?} is invalid")))?;
        // ensure the validator is not optimized out
        // and the drop is called on the secret wrapper.
        std::hint::black_box(secret);
        Ok(self)
    }
}

// The wrapper is needed to guarantee zeroization of the `iban::Iban` object used by validator
struct Secret(Iban);

impl Drop for Secret {
    fn drop(&mut self) {
        unsafe {
            let iban_str = self.0.electronic_str();
            let ptr = iban_str.as_ptr() as *mut u8;
            // Under the hood the `iban_validate` crate uses `ArrayString<34>` to store the IBAN,
            // and the `electronic_str()` method returns a reference to this array.
            std::ptr::write_bytes(ptr, 0, 34);
        }
    }
}

// SAFETY: The trait is safely implemented because exposing the first 2 and last 4 characters:
// 1. Neither causes out-of-bounds access to potentially INVALID (empty) data,
//    due to fallbacks to the empty strings,
// 2. Nor leaks the essential part of the sensitive VALID IBAN (which has at least 15 chars)
//    even though the first chars can decide its actual length.
unsafe impl Masked for IBAN {
    const TYPE_WRAPPER: &'static str = "IBAN";

    #[inline]
    fn first_chars(&self) -> String {
        self.0.get(0..2).unwrap_or_default().to_string()
    }

    #[inline]
    fn last_chars(&self) -> String {
        let len = self.0.len();
        self.0.get(len - 4..len).unwrap_or_default().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_IBAN_DE: &str = "DE89370400440532013000";
    const VALID_IBAN_GB: &str = "GB82WEST12345698765432";

    mod construction {
        use super::*;

        #[test]
        fn accepts_valid_ibans() {
            for input in [VALID_IBAN_DE, VALID_IBAN_GB, "FR1420041010050500013M02606"] {
                let result = IBAN::try_from(input);
                assert!(result.is_ok(), "{input:?} failed validation");
            }
        }

        #[test]
        fn removes_separators() {
            let input = " DE89 3704 0044 0532 0130 00 \n\t\r ";
            let iban = IBAN::try_from(input).unwrap();
            let result = unsafe { iban.as_ref() };
            assert_eq!(result, VALID_IBAN_DE);
        }

        #[test]
        fn rejects_invalid_iban_checksum() {
            let input = "DE89370400440532013001"; // Wrong checksum
            let result = IBAN::try_from(input);

            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_invalid_iban_format() {
            let input = "INVALID123456789";
            let result = IBAN::try_from(input);

            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_too_short_iban() {
            let input = "DE89";
            let result = IBAN::try_from(input);

            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }
    }

    mod safety {
        use super::*;

        #[test]
        fn masks_debug() {
            let iban = IBAN::try_from(VALID_IBAN_DE).unwrap();
            let debug_output = format!("{:?}", iban);
            assert!(debug_output.contains(r#"IBAN("DE***3000")"#));
        }

        #[test]
        fn as_ref_is_unsafe() {
            static_assertions::assert_not_impl_all!(IBAN: AsRef<str>);

            let input = "DE89 3704 0044 0532 0130 00";
            let iban = IBAN::try_from(input).unwrap();
            let exposed = unsafe { <IBAN as AsUnsafeRef<str>>::as_ref(&iban) };
            assert_eq!(exposed, VALID_IBAN_DE);
        }

        #[test]
        fn memory_is_not_leaked_after_drop() {
            let ptr: *const u8;
            let len: usize;
            unsafe {
                let iban = IBAN::try_from(VALID_IBAN_DE).unwrap();
                let s = iban.as_ref();
                ptr = s.as_ptr();
                len = s.len();
            }

            // SAFETY: This test verifies memory was zeroed after a drop.
            // Reading potentially freed memory is unsafe and only valid in tests
            // immediately after a drop, before any reallocation.
            unsafe {
                let slice = std::slice::from_raw_parts(ptr, len);
                let original_bytes = VALID_IBAN_DE.as_bytes();
                assert_ne!(
                    slice, original_bytes,
                    "Original IBAN should not remain in memory after drop"
                );
            }
        }
    }
}
