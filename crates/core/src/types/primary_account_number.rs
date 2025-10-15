use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;
use zeroize_derive::ZeroizeOnDrop;

use crate::error::Error;
use crate::internal::{HighlySecret, Masked, sanitized::*, validated::*};

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
/// * exposed via the **unsafe** `with_exposed_secret` method only,
///   forcing gateway developers to acknowledge the handling of sensitive data.
#[derive(Clone, ZeroizeOnDrop)]
pub struct PrimaryAccountNumber(String);

impl FromStr for PrimaryAccountNumber {
    type Err = Error;

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::sanitize(input).validated()
    }
}

impl<'a> HighlySecret<'a> for PrimaryAccountNumber {
    type Exposed = &'a str;

    #[inline]
    unsafe fn with_exposed_secret<T, F>(&'a self, f: F) -> T
    where
        F: FnOnce(Self::Exposed) -> T,
    {
        f(self.0.as_str())
    }
}

impl fmt::Debug for PrimaryAccountNumber {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as Masked>::masked_debug(self, f)
    }
}

// --- Sealed traits (not parts of the public API) ---

impl<'a> Sanitized<'a> for PrimaryAccountNumber {
    type Input = &'a str;

    #[inline]
    fn sanitize(input: Self::Input) -> Self {
        let mut output = Self(String::with_capacity(input.len()));
        filter_characters(&mut output.0, input, "-_");
        output
    }
}

impl Validated for PrimaryAccountNumber {
    #[inline]
    fn validate(&self) -> Result<(), String> {
        validate_length(&self.0, 13, 19)?;
        validate_alphanumeric(&self.0, "")?;

        if self.0.starts_with('0') {
            Err("cannot start with 0".to_string())?;
        }
        // Safety: under the hood the function copies bytes from the input,
        //         but assign them to the same variable/memory location
        //         during the iteration cycle, so only the last char is left
        //         in the stack memory without zeroization.
        if !luhn3::valid(self.0.as_bytes()) {
            Err("failed the Luhn check".to_string())?;
        }

        Ok(())
    }
}

// SAFETY: The trait is safely implemented because exposing the last 4 characters:
// 1. Neither causes out-of-bounds access to potentially INVALID (empty) data,
//    due to fallbacks to the empty strings,
// 2. Nor leaks the essential part of the sensitive VALID data which has at least 13 chars
//    (and this is explicitly enabled by the PCI DSS requirements).
unsafe impl Masked for PrimaryAccountNumber {
    const TYPE_WRAPPER: &'static str = "PrimaryAccountNumber";

    #[inline]
    fn last_chars(&self) -> String {
        let len = self.0.len();
        self.0.get(len - 4..len).unwrap_or_default().to_uppercase()
    }
}
