use std::fmt;
use zeroize_derive::ZeroizeOnDrop;

use crate::internal::{Masked, Validated};
use crate::{AsUnsafeRef, Error};

/// Token representing a stored credential from a payment gateway
///
/// Represents stored payment method identifiers used for recurring payments,
/// including SEPA/ACH mandates, card tokens, and setup intents.
///
/// # Validation
/// * length: 8-255 characters
/// * ASCII alphanumeric characters, dashes, and underscores only
/// * no leading or trailing whitespace
///
/// # Coverage
/// Supports payment systems worldwide:
/// * Stripe: `pm_1Q0PsIJvEtkwdCNYMSaVuRz6` (payment method)
/// * GoCardless: `MD0000S2KNTB2V` (mandate)
/// * Adyen: `8313147988756818` (storedPaymentMethodId)
/// * PAYONE: `9410010000038248746` (pseudocardpan)
/// * Unzer: `s-crd-fm7tifzkqewy` (payment type)
/// * Mollie: `mdt_pWUnw6pkBN` (mandate)
/// * Dwolla: `fc84223a-609f-42c9-866e-2c98f17ab4fb` (funding source)
/// * Gr4vy: `ef9496d8-53a5-4aad-8ca2-00eb68334389` (payment method)
/// * Spreedly: `PTp0nIk2NcqxaTlgsx3Esz2JSAN` (payment method)
/// * Razorpay: `token_4lsdksD31GaZ09` (token)
/// * Xendit: `pm-6ff0b6f2-f5de-457f-b08f-bc98fbae485a` (payment method)
///
/// # Data Protection
/// Stored credential tokens enable transaction correlation and can be used to initiate debits.
/// As such, they are:
/// * masked in logs (via `Debug` implementation) to display the last 4 characters only
/// * not exposed publicly except via **unsafe** method `as_ref`
#[derive(Clone, ZeroizeOnDrop)]
pub struct StoredCredentialToken(String);

impl<'a> TryFrom<&'a str> for StoredCredentialToken {
    type Error = Error;

    #[inline]
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        Self(input.to_string()).validate()
    }
}

impl fmt::Debug for StoredCredentialToken {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.masked_debug(f)
    }
}

impl AsUnsafeRef<str> for StoredCredentialToken {
    #[inline]
    unsafe fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

// --- Sealed traits (not parts of the public API) ---

impl Validated for StoredCredentialToken {
    #[inline]
    fn validate(self) -> Result<Self, Error> {
        self._validate_length(&self.0, 8, 255)?;
        self._validate_alphanumeric(&self.0, "-_")?;
        self._validate_no_trailing_spaces(&self.0)?;
        Ok(self)
    }
}

// SAFETY: The trait is safely implemented because exposing only the last 4 characters:
// 1. Neither causes out-of-bounds access to potentially INVALID (empty) data,
//    due to fallbacks to the empty strings,
// 2. Nor leaks the essential part of the sensitive VALID data which has at least 8 chars.
unsafe impl Masked for StoredCredentialToken {
    const TYPE_WRAPPER: &'static str = "StoredCredentialToken";

    #[inline]
    fn first_chars(&self) -> String {
        String::new()
    }

    #[inline]
    fn last_chars(&self) -> String {
        self.0
            .get(self.0.len().saturating_sub(4)..)
            .unwrap_or_default()
            .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_TOKEN: &str = "pm_12345678";

    mod construction {
        use super::*;

        #[test]
        fn accepts_valid_formats() {
            let formats = [
                ("pm_1MowQVLkdIwHu7ix", "Stripe"),
                ("MD00003GKMHFFY", "GoCardless"),
                ("mdt_h3gAaD5zP", "Mollie"),
                ("token_DZBYFa6RM5pMGb", "Razorpay"),
                ("pm-6ff0b6f2-f5de-457f-b08f-bc98fbae485a", "Xendit"),
                ("77a76f7e-d2de-4bbc-ada9-d6a0015e6bd5", "Gr4vy UUID"),
                ("s-sdd-fm7tifzkqewy", "Unzer"),
                ("12345678", "min length"),
                (&"a".repeat(255), "max length"),
            ];

            for (input, name) in formats {
                let result = StoredCredentialToken::try_from(input);
                assert!(result.is_ok(), "{name} format failed: {input}");
            }
        }

        #[test]
        fn rejects_surrounding_whitespace() {
            let inputs = [" pm_12345678", "pm_12345678 ", " pm_12345678 "];
            for input in inputs {
                let result = StoredCredentialToken::try_from(input);
                assert!(matches!(result, Err(Error::InvalidInput(_))));
            }
        }

        #[test]
        fn rejects_too_short_token() {
            let result = StoredCredentialToken::try_from("1234567");
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }

        #[test]
        fn rejects_too_long_token() {
            let input = "a".repeat(256);
            let result = StoredCredentialToken::try_from(input.as_str());
            assert!(matches!(result, Err(Error::InvalidInput(_))));
        }
    }

    mod safety {
        use super::*;

        #[test]
        fn masks_debug() {
            let token = StoredCredentialToken::try_from(VALID_TOKEN).unwrap();
            let debug_output = format!("{:?}", token);
            assert!(debug_output.contains(r#"StoredCredentialToken("***5678")"#));
        }

        #[test]
        fn memory_is_not_leaked_after_drop() {
            let ptr: *const u8;
            let len: usize;
            unsafe {
                let token = StoredCredentialToken::try_from(VALID_TOKEN).unwrap();
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
