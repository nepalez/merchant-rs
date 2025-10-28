use std::fmt;
use zeroize_derive::ZeroizeOnDrop;

use crate::Error;
use crate::internal::{Masked, Validated, sanitized::*};

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

impl AsRef<str> for AccountNumber {
    #[inline]
    fn as_ref(&self) -> &str {
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
