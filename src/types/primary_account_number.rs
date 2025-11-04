use std::convert::TryFrom;
use std::fmt;
use zeroize_derive::ZeroizeOnDrop;

use crate::Error;
use crate::internal::{AsUnsafeRef, Masked, Validated, sanitized::*};

/// Primary account number (PAN) from a payment card
///
/// # Sanitization
/// * removes dashes, underscores and spaces (common separators),
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * length: 13-19 characters,
/// * only digits are allowed,
/// * cannot start with 0,
/// * must pass the Luhn check (Mod 10)
///
/// # Data Protection
/// PCI DSS classifies PAN as sensitive authentication data (SAD) that provides
/// full access to cardholder funds and enables fraudulent transactions.
///
/// As such, it is:
/// * fully masked in logs (via `Debug` implementation) to prevent any leaks,
/// * not exposed publicly except for a part of a request or response
///   via **unsafe** method `with_exposed_secret`.
#[derive(Clone, ZeroizeOnDrop)]
pub struct PrimaryAccountNumber(String);

impl<'a> TryFrom<&'a str> for PrimaryAccountNumber {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self::sanitize(input).validate()
    }
}

impl fmt::Debug for PrimaryAccountNumber {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as Masked>::masked_debug(self, f)
    }
}

impl AsUnsafeRef<str> for PrimaryAccountNumber {
    #[inline]
    unsafe fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for PrimaryAccountNumber {
    #[inline]
    fn sanitize(input: &str) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        filter_characters(&mut output.0, input, "-_");
        output
    }
}

impl Validated for PrimaryAccountNumber {
    fn validate(self) -> Result<Self, Error> {
        self._validate_length(&self.0, 13, 19)?;
        self._validate_alphanumeric(&self.0, "")?;

        if self.0.starts_with('0') {
            Err(Error::InvalidInput(format!("{self:?} cannot start with 0")))
        } else if !luhn3::valid(self.0.as_bytes()) {
            Err(Error::InvalidInput(format!(
                "{self:?} failed the Luhn check"
            )))
        } else {
            Ok(self)
        }
    }
}

/// # Safety
/// The trait is safely implemented because exposing the first 1 and the last 4 characters:
/// 1. Neither causes out-of-bounds access to potentially INVALID (empty) data,
///    due to fallbacks to the empty strings,
/// 2. Nor leaks the essential part of the sensitive VALID data which has at least 13 chars
///    (and this is explicitly enabled by the PCI DSS requirements).
unsafe impl Masked for PrimaryAccountNumber {
    const TYPE_WRAPPER: &'static str = "PrimaryAccountNumber";

    #[inline]
    fn first_chars(&self) -> String {
        self.0.get(0..1).unwrap_or_default().to_string()
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

    const VALID_VISA_16: &str = "4532015112830366";
    const VALID_MASTERCARD_16: &str = "5425233430109903";
    const VALID_AMEX_15: &str = "374245455400126";
    const VALID_VISA_13: &str = "4222222222222";
    const VALID_DISCOVER_19: &str = "6011000990139424389";

    mod construction {
        use super::*;

        #[test]
        fn accepts_valid_numbers() {
            for input in [
                VALID_VISA_13,
                VALID_VISA_16,
                VALID_MASTERCARD_16,
                VALID_AMEX_15,
                VALID_DISCOVER_19,
            ] {
                let result = PrimaryAccountNumber::try_from(input);
                assert!(result.is_ok(), "{input:?} failed validation");
            }
        }

        #[test]
        fn removes_separators() {
            let input = " 4532-0151_1283 0366\n\t\r ";
            let pan = PrimaryAccountNumber::try_from(input).unwrap();
            let result = unsafe { pan.as_ref() };
            assert_eq!(result, VALID_VISA_16);
        }

        #[test]
        fn rejects_too_short_number() {
            let input = "123456789012"; // 12 digits
            let result = PrimaryAccountNumber::try_from(input);

            if let Err(Error::InvalidInput(msg)) = result {
                assert!(msg.contains("1***9012"));
            } else {
                panic!("Expected InvalidInput error, got {result:?}");
            }
        }

        #[test]
        fn rejects_too_long_number() {
            let input = "12345678901234567890"; // 20 digits
            let result = PrimaryAccountNumber::try_from(input);

            if let Err(Error::InvalidInput(msg)) = result {
                assert!(msg.contains("1***7890"));
            } else {
                panic!("Expected InvalidInput error, got {result:?}");
            }
        }

        #[test]
        fn rejects_non_numeric_characters() {
            let input = "453201511283036A";
            let result = PrimaryAccountNumber::try_from(input);

            if let Err(Error::InvalidInput(msg)) = result {
                assert!(msg.contains("4***036A"));
            } else {
                panic!("Expected InvalidInput error, got {result:?}");
            }
        }

        #[test]
        fn rejects_number_starting_with_zero() {
            let input = "0532015112830366";
            let result = PrimaryAccountNumber::try_from(input);

            if let Err(Error::InvalidInput(msg)) = result {
                assert!(msg.contains("0***0366"));
            } else {
                panic!("Expected InvalidInput error, got {result:?}");
            }
        }

        #[test]
        fn rejects_number_failing_luhn_check() {
            let input = "4532015112830367"; // Changed last digit - fails Luhn
            let result = PrimaryAccountNumber::try_from(input);

            if let Err(Error::InvalidInput(msg)) = result {
                assert!(msg.contains("4***0367"));
            } else {
                panic!("Expected InvalidInput error");
            }
        }
    }

    mod safety {
        use super::*;

        #[test]
        fn masks_debug() {
            let pan = PrimaryAccountNumber::try_from(VALID_VISA_16).unwrap();
            let debug_output = format!("{:?}", pan);
            assert!(debug_output.contains(r#"PrimaryAccountNumber("4***0366")"#));
        }

        #[test]
        fn as_ref_is_unsafe() {
            static_assertions::assert_not_impl_all!(PrimaryAccountNumber: AsRef<str>);

            let input = "4532-0151-1283-0366";
            let pan = PrimaryAccountNumber::try_from(input).unwrap();
            let exposed = unsafe { <PrimaryAccountNumber as AsUnsafeRef<str>>::as_ref(&pan) };
            assert_eq!(exposed, VALID_VISA_16);
        }

        #[test]
        fn memory_is_not_leaked_after_drop() {
            let ptr: *const u8;
            let len: usize;
            unsafe {
                let pan = PrimaryAccountNumber::try_from(VALID_VISA_16).unwrap();
                let s = pan.as_ref();
                ptr = s.as_ptr();
                len = s.len();
            }

            // SAFETY: This test verifies memory was zeroed after a drop.
            // Reading potentially freed memory is unsafe and only valid in tests
            // immediately after a drop, before any reallocation.
            unsafe {
                let slice = std::slice::from_raw_parts(ptr, len);
                let original_bytes = VALID_VISA_16.as_bytes();
                assert_ne!(
                    slice, original_bytes,
                    "Original card number should not remain in memory after drop"
                );
            }
        }
    }
}
