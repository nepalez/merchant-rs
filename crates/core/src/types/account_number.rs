use std::fmt;
use std::str::FromStr;
use zeroize_derive::ZeroizeOnDrop;

use crate::error::Error;
use crate::internal::{HighlySecret, Masked, sanitized::*, validated::*};

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
/// * exposed via the **unsafe** `with_exposed_secret` method only,
///   forcing gateway developers to acknowledge the handling of sensitive data.
#[derive(Clone, ZeroizeOnDrop)]
pub struct AccountNumber(String);

impl FromStr for AccountNumber {
    type Err = Error;

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::sanitize(input).validated()
    }
}

impl fmt::Debug for AccountNumber {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as Masked>::masked_debug(self, f)
    }
}

impl<'a> HighlySecret<'a> for AccountNumber {
    type Exposed = &'a str;

    #[inline]
    unsafe fn with_exposed_secret<T, F>(&'a self, f: F) -> T
    where
        F: FnOnce(Self::Exposed) -> T,
    {
        f(self.0.as_str())
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
    fn validate(&self) -> Result<(), String> {
        validate_length(&self.0, 4, 20)?;
        validate_alphanumeric(&self.0, "")
    }
}

// SAFETY: The trait is safely implemented as it does NOT expose any part of the internal value.
unsafe impl Masked for AccountNumber {
    const TYPE_WRAPPER: &'static str = "AccountNumber";
}
