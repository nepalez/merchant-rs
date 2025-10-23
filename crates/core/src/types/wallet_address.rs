use std::fmt;
use std::str::FromStr;
use zeroize_derive::ZeroizeOnDrop;

use crate::Error;
use crate::internal::{Masked, PersonalData, sanitized::*, validated::*};

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
/// * exposed via the **unsafe** `as_str` method only,
///   forcing gateway developers to acknowledge the handling of sensitive data.
#[derive(Clone, ZeroizeOnDrop)]
pub struct WalletAddress(String);

impl FromStr for WalletAddress {
    type Err = Error;

    #[inline]
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Self::sanitize(input).validated()
    }
}

impl fmt::Debug for WalletAddress {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as Masked>::masked_debug(self, f)
    }
}

impl PersonalData for WalletAddress {
    #[inline]
    unsafe fn as_str(&self) -> &str {
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
    fn validate(&self) -> Result<(), String> {
        validate_length(&self.0, 20, 90)
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
            .get(self.0.len() - 6..)
            .unwrap_or_default()
            .to_string()
    }
}
