use std::convert::TryFrom;
use std::fmt;
use zeroize_derive::ZeroizeOnDrop;

use crate::internal::{Masked, Validated, sanitized::*};
use crate::{AsUnsafeRef, Error};

/// Virtual Payment Address (UPI, PIX)
///
/// # Sanitization
/// * trims whitespaces,
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * length: 7-255 characters
///
/// # Data Protection
/// While virtual payment addresses are publicly accessible,
/// they can be used to identify users when combined with other data,
/// making them PII (Personal Identifiable Information).
///
/// As such, they are:
/// * masked in logs (via `Debug` implementation) to display
///   the first character and either the domain part (for UPI),
///   or the last 3 characters (for PIX),
/// * not exposed publicly except for a part of a request or response
///   via **unsafe** method `with_exposed_secret`.
#[derive(Clone, ZeroizeOnDrop)]
pub struct VirtualPaymentAddress(String);

impl<'a> TryFrom<&'a str> for VirtualPaymentAddress {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self::sanitize(input).validate()
    }
}

impl fmt::Debug for VirtualPaymentAddress {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.masked_debug(f)
    }
}

impl AsUnsafeRef<str> for VirtualPaymentAddress {
    #[inline]
    unsafe fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for VirtualPaymentAddress {
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

impl Validated for VirtualPaymentAddress {
    #[inline]
    fn validate(self) -> Result<Self, Error> {
        const MIN_UPI_LENGTH: usize = 7;
        const MAX_EMAIL_LENGTH: usize = 255;

        self._validate_length(&self.0, MIN_UPI_LENGTH, MAX_EMAIL_LENGTH)?;
        Ok(self)
    }
}

/// # Safety
/// The trait is safely implemented because exposing the first 1 character
/// along with the domain name (for UPI and emails in PIX) or the last 3 characters
/// for the other PIX identifiers (CPF, CNPJ, Phone, UUID):
/// 1. Neither causes out-of-bounds access to potentially INVALID (empty) data,
///    due to fallbacks to the empty strings,
/// 2. Nor leaks the real data due to hiding the real length of the email address
///    neither exposes the essential part of CPF (11 chars), CNPJ (14 chars), Phone (10 chars),
///    random UUID (36 chars).
///
/// # Warning
/// This masking is designed for logging/debugging purposes only.
/// LGPD compliance requires risk assessment: if your system has few users
/// or VPAs have unique structural patterns, even this partial masking may
/// allow re-identification. Perform a Data Protection Impact Assessment (DPIA)
/// to verify that re-identification risk remains acceptably low in your context.
/// Consider full redaction if a risk assessment indicates a high re-identification risk.
unsafe impl Masked for VirtualPaymentAddress {
    const TYPE_WRAPPER: &'static str = "VirtualPaymentAddress";

    #[inline]
    fn first_chars(&self) -> String {
        self.0.get(0..1).unwrap_or_default().to_string()
    }

    #[inline]
    fn last_chars(&self) -> String {
        if self.0.contains('@') {
            self.0.split_once('@').unwrap_or_default().1
        } else {
            self.0
                .get(self.0.len().saturating_sub(3)..)
                .unwrap_or_default()
        }
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_UPI: &str = "user@paytm";
    const VALID_PIX_CPF: &str = "12345678901";

    mod construction {
        use super::*;

        #[test]
        fn accepts_valid_addresses() {
            for input in [
                VALID_UPI,
                VALID_PIX_CPF,
                "a@b.com",
                "a".repeat(255).as_str(),
            ] {
                let result = VirtualPaymentAddress::try_from(input);
                assert!(result.is_ok(), "{input:?} failed validation");
            }
        }

        #[test]
        fn removes_control_characters() {
            let input = " user@paytm \n\t\r ";
            let vpa = VirtualPaymentAddress::try_from(input).unwrap();
            let result = unsafe { vpa.as_ref() };
            assert_eq!(result, VALID_UPI);
        }

        #[test]
        fn rejects_too_short_address() {
            let result = VirtualPaymentAddress::try_from("abc123");
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_too_long_address() {
            let input = "a".repeat(256);
            let result = VirtualPaymentAddress::try_from(input.as_str());
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }
    }

    mod safety {
        use super::*;

        #[test]
        fn masks_debug_upi() {
            let vpa = VirtualPaymentAddress::try_from(VALID_UPI).unwrap();
            let debug_output = format!("{:?}", vpa);
            // Masked format: first char + *** + domain after @
            assert!(debug_output.contains("VirtualPaymentAddress"));
            assert!(debug_output.contains("***"));
            assert!(debug_output.contains("paytm"));
        }

        #[test]
        fn masks_debug_pix() {
            let vpa = VirtualPaymentAddress::try_from(VALID_PIX_CPF).unwrap();
            let debug_output = format!("{:?}", vpa);
            assert!(debug_output.contains("1***901"));
        }

        #[test]
        fn memory_is_not_leaked_after_drop() {
            let ptr: *const u8;
            let len: usize;
            unsafe {
                let vpa = VirtualPaymentAddress::try_from(VALID_UPI).unwrap();
                let s = vpa.as_ref();
                ptr = s.as_ptr();
                len = s.len();
            }

            unsafe {
                let slice = std::slice::from_raw_parts(ptr, len);
                let original_bytes = VALID_UPI.as_bytes();
                assert_ne!(
                    slice, original_bytes,
                    "Original VPA should not remain in memory after drop"
                );
            }
        }
    }
}
