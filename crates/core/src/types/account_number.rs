use std::fmt;
use std::str::FromStr;
use zeroize_derive::ZeroizeOnDrop;

use crate::error::Error;
use crate::internal::{Exposed, sanitized::*, validated::*};

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
        <Self as Exposed>::masked_debug(self, f)
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

// SAFETY: The trait is safely implemented as:
// 1. it exposes a reference to the internal String which will be zeroized on a drop;
//    No copies are created, neither new memory is allocated;
// 2. it masks the total value in logs.
unsafe impl Exposed for AccountNumber {
    type Output<'a> = &'a str;

    const TYPE_WRAPPER: &'static str = "AccountNumber";

    #[inline]
    fn expose(&self) -> Self::Output<'_> {
        self.0.as_str()
    }
}
