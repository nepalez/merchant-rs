use iban::{Iban, IbanLike};
use std::fmt;
use std::str::FromStr;
use zeroize_derive::ZeroizeOnDrop;

use crate::error::Error;
use crate::internal::{Exposed, sanitized::*, validated::*};

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

const IBAN_CAPACITY: usize = 34;

impl FromStr for IBAN {
    type Err = Error;

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::sanitize(input).validated()
    }
}

impl fmt::Debug for IBAN {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.masked_debug(f)
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
    fn validate(&self) -> Result<(), String> {
        let secret = Iban::from_str(&self.0)
            .map(Secret)
            .map_err(|_| "is invalid".to_string())?;
        // ensure the validator is not optimized out
        // and the drop is called on the secret wrapper.
        std::hint::black_box(secret);
        Ok(())
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
unsafe impl Exposed for IBAN {
    type Output<'a> = &'a str;
    const TYPE_WRAPPER: &'static str = "IBAN";

    #[inline]
    fn expose(&self) -> Self::Output<'_> {
        self.0.as_str()
    }

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
