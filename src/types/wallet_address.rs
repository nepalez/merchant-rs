use std::convert::TryFrom;
use std::fmt;
use zeroize_derive::ZeroizeOnDrop;

use crate::Error;
use crate::internal::{AsUnsafeRef, Masked, Validated, sanitized::*};

/// Cryptocurrency wallet address
///
/// # Sanitization
/// * trims whitespaces,
/// * removes all ASCII control characters like newlines, tabs, etc.
///
/// # Validation
/// * length: 20-90 characters
///
/// # Data Protection
/// While wallet addresses are publicly accessible on blockchains,
/// they can be used to identify persons and track transaction history,
/// making them PII (Personal Identifiable Information).
///
/// As such, they are:
/// * masked in logs (via `Debug` implementation) to display
///   the first 6 and last 6 characters only,
/// * not exposed publicly except for a part of a request or response
///   via **unsafe** method `with_exposed_secret`.
#[derive(Clone, ZeroizeOnDrop)]
pub struct WalletAddress(String);

impl<'a> TryFrom<&'a str> for WalletAddress {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self::sanitize(input).validate()
    }
}

impl fmt::Debug for WalletAddress {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as Masked>::masked_debug(self, f)
    }
}

impl AsUnsafeRef<str> for WalletAddress {
    #[inline]
    unsafe fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Sanitized for WalletAddress {
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

impl Validated for WalletAddress {
    #[inline]
    fn validate(self) -> Result<Self, Error> {
        self._validate_length(&self.0, 20, 90)?;
        Ok(self)
    }
}

// SAFETY: The trait is safely implemented because exposing the first 6 and last 6 characters:
// 1. Neither causes out-of-bounds access to potentially INVALID (empty) data,
//    due to fallbacks to the empty strings,
// 2. Nor leaks the essential part of the sensitive VALID data which has at least 20 chars
//    (actually addresses have 26+ chars).
unsafe impl Masked for WalletAddress {
    const TYPE_WRAPPER: &'static str = "WalletAddress";

    #[inline]
    fn first_chars(&self) -> String {
        self.0.get(0..6).unwrap_or_default().to_string()
    }

    #[inline]
    fn last_chars(&self) -> String {
        self.0
            .get(self.0.len().saturating_sub(6)..)
            .unwrap_or_default()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_WALLET: &str = "1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa";

    mod construction {
        use super::*;

        #[test]
        fn accepts_valid_wallets() {
            for input in [VALID_WALLET, "a".repeat(20).as_str(), "a".repeat(90).as_str()] {
                let result = WalletAddress::try_from(input);
                assert!(result.is_ok(), "{input:?} failed validation");
            }
        }

        #[test]
        fn removes_control_characters() {
            let input = " 1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa \n\t\r ";
            let wallet = WalletAddress::try_from(input).unwrap();
            let result = unsafe { wallet.as_ref() };
            assert_eq!(result, VALID_WALLET);
        }

        #[test]
        fn rejects_too_short_wallet() {
            let result = WalletAddress::try_from("1234567890123456789");
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_too_long_wallet() {
            let input = "a".repeat(91);
            let result = WalletAddress::try_from(input.as_str());
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }
    }

    mod safety {
        use super::*;

        #[test]
        fn masks_debug() {
            let wallet = WalletAddress::try_from(VALID_WALLET).unwrap();
            let debug_output = format!("{:?}", wallet);
            assert!(debug_output.contains(r#"WalletAddress("1A1zP1***DivfNa")"#));
        }

        #[test]
        fn memory_is_not_leaked_after_drop() {
            let ptr: *const u8;
            let len: usize;
            unsafe {
                let wallet = WalletAddress::try_from(VALID_WALLET).unwrap();
                let s = wallet.as_ref();
                ptr = s.as_ptr();
                len = s.len();
            }

            unsafe {
                let slice = std::slice::from_raw_parts(ptr, len);
                let original_bytes = VALID_WALLET.as_bytes();
                assert_ne!(
                    slice, original_bytes,
                    "Original wallet address should not remain in memory after drop"
                );
            }
        }
    }
}
